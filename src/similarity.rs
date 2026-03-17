use std::collections::HashMap;

use crate::page_generation::{Post, PostSummary};

#[cfg(feature = "smart-similar")]
mod inner {
    use std::collections::HashMap;
    use std::time::Instant;

    use anyhow::Result;
    use model2vec_rs::model::StaticModel;

    use crate::page_generation::{Post, PostSummary};

    const MODEL_NAME: &str = "minishlab/potion-multilingual-128M";
    const EMBED_TEXT_CHAR_LIMIT: usize = 500;
    const TOP_SIMILAR_COUNT: usize = 3;

    pub fn compute_similar_posts(posts: &[Post]) -> Result<HashMap<String, Vec<PostSummary>>> {
        let start = Instant::now();
        println!("Loading embedding model ({})...", MODEL_NAME);
        let model = StaticModel::from_pretrained(MODEL_NAME, None, None, None)?;
        println!("  Model loaded in {:.2}s", start.elapsed().as_secs_f64());

        let texts: Vec<String> = posts
            .iter()
            .map(|p| build_embed_text(&p.title, &p.content_raw))
            .collect();

        let embed_start = Instant::now();
        let embeddings = model.encode(&texts);
        println!(
            "  Encoded {} posts in {:.2}s",
            posts.len(),
            embed_start.elapsed().as_secs_f64()
        );

        let mut similar_map = HashMap::with_capacity(posts.len());

        for (i, post) in posts.iter().enumerate() {
            let mut scored: Vec<(usize, f32)> = (0..posts.len())
                .filter(|&j| j != i)
                .map(|j| (j, cosine_similarity(&embeddings[i], &embeddings[j])))
                .collect();

            scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            scored.truncate(TOP_SIMILAR_COUNT);

            let summaries = scored
                .into_iter()
                .map(|(j, _)| post_to_summary(&posts[j]))
                .collect();

            similar_map.insert(post.id.clone(), summaries);
        }

        println!(
            "✓ Computed smart similarity in {:.2}s",
            start.elapsed().as_secs_f64()
        );

        Ok(similar_map)
    }

    fn build_embed_text(title: &str, raw_markdown: &str) -> String {
        let truncated: String = raw_markdown.chars().take(EMBED_TEXT_CHAR_LIMIT).collect();
        format!("{}. {}", title, truncated)
    }

    fn post_to_summary(post: &Post) -> PostSummary {
        PostSummary {
            id: post.id.clone(),
            title: post.title.clone(),
            date: post.date.clone(),
            tags: post.tags.clone(),
            icon: post.icon.clone(),
        }
    }

    fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
        let denominator = norm_a * norm_b;
        if denominator == 0.0 {
            return 0.0;
        }
        dot / denominator
    }
}

#[cfg(feature = "smart-similar")]
pub fn compute_similar_posts(posts: &[Post]) -> anyhow::Result<HashMap<String, Vec<PostSummary>>> {
    inner::compute_similar_posts(posts)
}

#[cfg(not(feature = "smart-similar"))]
pub fn compute_similar_posts(_posts: &[Post]) -> anyhow::Result<HashMap<String, Vec<PostSummary>>> {
    anyhow::bail!(
        "--smart-similar requires the 'smart-similar' feature. \
         Rebuild with: cargo build --features smart-similar"
    )
}
