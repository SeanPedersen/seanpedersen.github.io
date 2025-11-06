---
title: 'PostgreSQL Database'
date: '2025-10-13'
---
Postgres is an open-source "eierlegende Wollmilchsau" relational DBMS - which means it is capable of lots of things: spanning advanced search capabilities for text, vectors and geographic data and optimizations like partitioning and sharding. Understanding these optimization techniques can transform a moderately performing database into a high-performance system handling millions of queries per second with sub-millisecond latency.

Modern PostgreSQL deployments benefit from a holistic approach: vertical scaling through careful hardware selection and memory parameter tuning provides the foundation, while partitioning strategies enable efficient management of massive tables exceeding hundreds of gigabytes. Horizontal scaling through replication and connection pooling extends capacity beyond single-server limits. 

The transformation of PostgreSQL from general-purpose database to specialized search engine happens through extensions like PostGIS and PGVector. The key insight: PostgreSQL's extensibility means you rarely need external systems. Properly optimized PostgreSQL can handle workloads traditionally reserved for specialized databases like Elasticsearch, vector databases or GIS systems. This reduces operational complexity while maintaining ACID guarantees and familiar SQL interfaces.

Modern PostgreSQL deployments demonstrate remarkable scalability: being used by major players like OpenAI, Spotify and Netflix to power mission-critical workloads that demand both high throughput and strong consistency. Its advanced replication features, such as logical and streaming replication, allow organizations to scale reads horizontally and maintain near-zero downtime during upgrades.

## Database Maintenance

PostgreSQL's Multi-Version Concurrency Control (MVCC) architecture creates dead tuples whenever rows are updated or deleted. An UPDATE operation actually performs a DELETE followed by INSERT, leaving the old row version behind. These dead tuples accumulate over time, consuming disk space and degrading query performance. **VACUUM reclaims this space and prevents transaction ID wraparound**, a catastrophic failure mode that occurs when PostgreSQL's 32-bit transaction counter approaches its 4 billion limit.

Standard VACUUM runs concurrently with normal operations, acquiring only a ShareUpdateExclusive lock that permits reads and writes. It marks space occupied by dead tuples as reusable within the same table but doesn't return disk space to the operating system. This process also "freezes" old tuples by marking them visible to all future transactions, preventing wraparound. Configure autovacuum with appropriate thresholds to ensure it runs frequently enough—the default 20% scale factor means a 1TB table accumulates 200GB of dead tuples before vacuuming triggers, which is far too permissive for large tables.

VACUUM FULL rewrites the entire table into a new disk file with no wasted space, returning freed disk space to the operating system. However, it requires an ACCESS EXCLUSIVE lock that blocks all operations and needs up to 2x the table's size in temporary disk space. **Run VACUUM FULL only during scheduled maintenance windows for extreme bloat situations**, never as routine maintenance. For a 100GB table with 90% bloat, VACUUM FULL might take hours and block all access. Instead, prevent bloat accumulation through proper autovacuum tuning.

Critical autovacuum parameters require adjustment from PostgreSQL defaults. Set `autovacuum_vacuum_scale_factor` to 0.01 (1%) instead of the default 0.2 (20%) for tables exceeding 100GB. This triggers vacuum after 10GB of dead tuples instead of 200GB. Increase `autovacuum_vacuum_cost_limit` from 200 to 2000 or higher on modern hardware—PostgreSQL 14+ improved cost calculations dramatically, providing roughly 10x higher throughput. For high-write tables, configure per-table settings: `ALTER TABLE busy_table SET (autovacuum_vacuum_scale_factor = 0.01, autovacuum_vacuum_cost_limit = 2000)`.

Monitor vacuum effectiveness with `pg_stat_user_tables`. Check `n_dead_tup` and `last_autovacuum` columns regularly. Calculate bloat ratio with `n_dead_tup / (n_live_tup + n_dead_tup) * 100`—healthy tables maintain below 5-10% dead tuples. Long-running transactions block VACUUM from removing dead tuples visible to those transactions. Query `pg_stat_activity` for transactions running longer than 5 minutes and investigate: `SELECT pid, now() - xact_start AS duration, query FROM pg_stat_activity WHERE state = 'active' AND (now() - xact_start) > interval '5 minutes'`.

Essential psql commands streamline database administration. Use `\dt+` to list tables with sizes, `\di+` for indexes with sizes and `\x auto` to automatically switch to vertical display for wide results. Create useful shortcuts in `~/.psqlrc` for common operations. Enable timing with `\timing on` to measure query performance. The `\watch` command repeats queries at intervals, useful for monitoring: `SELECT * FROM pg_stat_activity WHERE state != 'idle' \watch 5`.

Performance monitoring through SQL queries provides operational visibility. Check buffer cache hit ratios—values below 99% indicate insufficient `shared_buffers` or queries scanning too much data. Monitor index usage with `pg_stat_user_indexes` to identify unused indexes consuming disk space and write performance. Track query performance with the `pg_stat_statements` extension, which aggregates statistics across query patterns.

Backups: <https://github.com/RostislavDugin/postgresus/>

## Scaling & Optimisations

### Vertical Scaling (bigger server)

Correct hardware selection establishes the foundation for PostgreSQL performance. Modern PostgreSQL scales effectively across 64+ CPU cores for read workloads and ~20 cores for write-heavy operations. **NVMe storage delivers 2.4-3x faster query performance than network-attached storage**, with sub-millisecond latency compared to 1-3ms for SATA SSDs. A $600 NVMe drive provides 2.5 million IOPS—equivalent to cloud storage costing $1.3 million monthly. Configure `random_page_cost = 1.0` for SSDs instead of the default 4.0 calibrated for spinning disks and set `effective_io_concurrency = 200` for NVMe to enable parallel I/O operations.

Memory configuration represents the highest-impact tuning area. Set `shared_buffers` to 25% of total RAM as a starting point, maximum 40% on dedicated database servers. Beyond 40%, diminishing returns appear because PostgreSQL relies on the operating system's page cache as a second layer of caching. A 64GB RAM system typically performs best with 16GB shared_buffers. Enable huge pages (`huge_pages = on`) when shared_buffers exceeds 8GB to reduce page table overhead.

The `work_mem` parameter controls memory for each query operation—sorts, hash tables and merge joins. Each complex query might use this allocation multiple times. **A query with 4 parallel workers performing 2 hash operations potentially allocates 4 × 2 × work_mem memory**. Calculate conservatively: `work_mem = (Total RAM × 0.25) / max_connections`. For a 32GB system with 100 connections, set work_mem to 80MB. Override this per-session for analytical queries: `SET work_mem = '256MB'` before running complex aggregations. Monitor temporary file creation with `log_temp_files = 0` to identify queries spilling to disk that need higher work_mem.

Configure `maintenance_work_mem` to 5% of total RAM, typically 1-2GB. This accelerates VACUUM, CREATE INDEX and bulk loading operations. Autovacuum uses a separate `autovacuum_work_mem` parameter—set this lower than maintenance_work_mem since multiple autovacuum workers run concurrently. Set `effective_cache_size` to 50-75% of total RAM to help the query planner understand total available cache memory (PostgreSQL + OS combined).

Parallel query execution multiplies CPU utilization for analytical workloads. Configure `max_worker_processes` to match CPU core count, setting the pool from which all parallel workers draw. Set `max_parallel_workers_per_gather` to 4-8 for mixed workloads or 8-16 for pure analytics. A 16-core system handling data warehouse queries might configure: `max_worker_processes = 16`, `max_parallel_workers = 12`, `max_parallel_workers_per_gather = 8`. Real-world benchmarks show 400GB scans completing in 88 seconds with 10 parallel workers versus 290 seconds single-threaded—a **3.3x speedup**.

#### PGBench

Run pgbench to eval server performance quickly.

```bash
sudo -u postgres psql -c "CREATE DATABASE testdb;"
sudo -u postgres pgbench -i -s 10 testdb  # -s 10 = 10M rows
sudo -u postgres pgbench -c 10 -j 2 -T 60 testdb # -c clients, -j threads, -T duration(sec)
```

#### TABLESPACE

TABLESPACE maps tables and indexes to specific storage volumes - useful for placing hot data on NVMe and archival data on cheaper disks (reduce costs and optimize performance).

Create new tablespace:
```sql
CREATE TABLESPACE fastssd LOCATION '/mnt/hetzner-volume';
```

Move existing table and index to tablespace:
```sql
ALTER TABLE table_name SET TABLESPACE fastssd;
ALTER INDEX table_name_idx SET TABLESPACE fastssd;
```

### Table Partitioning Strategies

Table partitioning divides large tables into smaller physical pieces while maintaining a single logical table interface. Partitioning becomes valuable when tables exceed 100GB and contain natural segmentation boundaries in query patterns. For example if the data in a table is frequently queried by a language id and the language association of items does not change frequently, it makes sense to partition the table by language id.

Range partitioning splits data by value ranges, ideal for time-series data. Create a parent table with `PARTITION BY RANGE (timestamp_column)`, then add child partitions for each time period. Monthly partitions work well for event logs accumulating gigabytes daily. PostgreSQL 11+ supports declarative partitioning with automatic constraint management. Include the partition key in the primary key: `PRIMARY KEY (event_time, id)` ensures uniqueness constraints work across partitions.

```sql
CREATE TABLE events (
    id BIGSERIAL,
    event_time TIMESTAMPTZ NOT NULL,
    payload JSONB,
    PRIMARY KEY (event_time, id)
) PARTITION BY RANGE (event_time);

CREATE TABLE events_2024_01 PARTITION OF events
    FOR VALUES FROM ('2024-01-01') TO ('2024-02-01');
    
CREATE INDEX ON events_2024_01 (event_time);
```

List partitioning organizes data by discrete values like geographic regions or tenant IDs in multi-tenant systems. Define explicit value lists for each partition: `FOR VALUES IN ('USA', 'CANADA', 'MEXICO')`. Create a DEFAULT partition to capture unlisted values, though adding new partitions when a DEFAULT exists requires scanning the DEFAULT partition to move matching rows.

Hash partitioning distributes rows evenly across partitions using a hash function, preventing hotspots when no natural ordering exists. Create with `PARTITION BY HASH (customer_id)` and define partitions with modulus and remainder: `FOR VALUES WITH (MODULUS 4, REMAINDER 0)`. **Hash partitioning reduced memory usage at Cubbit when they decreased from 256 to 64 partitions** while maintaining query performance improvements from partition pruning.

Partition pruning eliminates scanning irrelevant partitions at query planning or execution time. Queries must include the partition key in WHERE clauses using immutable expressions. `WHERE event_time >= '2024-01-01' AND event_time < '2024-02-01'` enables pruning, but `WHERE event_time > NOW() - INTERVAL '1 day'` prevents it because NOW() is not immutable. Verify pruning with EXPLAIN—look for scans on specific partitions rather than all partitions.

Choose partition counts carefully. Too few partitions (under 10) provide minimal benefits. Too many partitions (thousands) increase query planning time exponentially and consume excessive memory. Aim for several dozen to a few hundred partitions maximum. **PostgreSQL 12 improved partition handling significantly**, enabling efficient management of thousands of partitions where previously 100 was the practical limit.

Index creation on partitioned tables automatically propagates to all child partitions. Create indexes on the parent table after creating all partitions for efficiency. Use `CREATE INDEX CONCURRENTLY` in production to avoid locking. For large partitions, create indexes concurrently on each partition individually, then attach to the parent index to maintain service availability.

Partition maintenance requires automation. Use pg_partman extension for automatic partition creation and removal based on retention policies. Detaching old partitions provides instant deletion: `ALTER TABLE events DETACH PARTITION events_2020_01; DROP TABLE events_2020_01;`. This avoids the bloat and performance degradation of deleting millions of rows individually. Cubbit achieved instant deletion of expired data through partition drops versus expensive DELETE operations scanning entire tables.

Performance benchmarks show partitioning trades increased write overhead for faster targeted queries. Range queries with proper partition pruning execute up to 10x faster by scanning only relevant partitions. Point queries incur 15-20% overhead due to planning complexity. Bulk loads run 20-25% slower with 400+ partitions due to partition switching costs. Optimize by loading data pre-sorted by partition key.

### Horizontal Scaling (more servers)

Streaming replication provides the foundation for PostgreSQL horizontal scaling. The primary server continuously sends Write-Ahead Log (WAL) changes to standby servers, which replay transactions to maintain synchronized copies. Configure `wal_level = replica` and `max_wal_senders = 10` on the primary. Create a replication user with `CREATE ROLE replication_user WITH REPLICATION LOGIN`. Initialize standbys using `pg_basebackup` to clone the primary's data directory, then configure `primary_conninfo` pointing to the primary server.

**Asynchronous replication** delivers superior performance by not waiting for standby confirmation before committing transactions. This introduces potential data loss if the primary fails before standbys receive recent transactions. Synchronous replication (`synchronous_commit = on`) waits for at least one standby to acknowledge WAL writes before committing, trading performance for zero data loss. Configure multiple synchronous standbys with quorum: `synchronous_standby_names = 'ANY 2 (standby1, standby2, standby3)'` requires any two standbys to acknowledge before committing.

Read replicas offload SELECT queries from the primary, horizontally scaling read capacity. Configure multiple standbys, then route read traffic across them using connection pooling or application-level load balancing. HAProxy provides effective round-robin distribution with health checks. Monitor replication lag with `pg_stat_replication` on the primary—check `write_lag`, `flush_lag` and `replay_lag` columns. Aim for lag under 1 second for local replicas, under 5 seconds for cross-region. High lag indicates network issues, under-provisioned replica hardware, or long-running transactions blocking WAL replay.

Logical replication enables selective table replication and cross-version replication, unlike physical streaming replication's full-cluster approach. Create publications on the source: `CREATE PUBLICATION my_pub FOR TABLE users, orders`. Create subscriptions on the target: `CREATE SUBSCRIPTION my_sub CONNECTION 'host=source_db' PUBLICATION my_pub`. Logical replication supports heterogeneous targets, enabling blue-green deployments and gradual migrations.

**PgBouncer provides connection pooling, allowing 10-50x more client connections** than direct PostgreSQL connections. PostgreSQL's process-per-connection model consumes 5-10MB per connection; with 1000 connections, that's 5-10GB overhead. PgBouncer runs as a lightweight proxy, maintaining a small pool of server connections while accepting thousands of client connections. Configure `pool_mode = transaction` for optimal concurrency—connections return to the pool after each transaction completes, maximizing reuse.

```ini
[pgbouncer]
pool_mode = transaction
default_pool_size = 25
max_client_conn = 1000
reserve_pool_size = 5
server_idle_timeout = 600
```

Calculate pool size using `(num_cores × 2) + effective_spindle_count`. An 8-core system with SSD typically needs 20 server connections. Allow 50x more client connections than server connections through pooling. Transaction mode provides 2-3x throughput improvement compared to direct connections. Limitations include incompatibility with session-level features like temporary tables persisting across transactions and SET parameters outside transactions.

Citus extension enables sharding—distributing data across multiple PostgreSQL servers. Install with `CREATE EXTENSION citus`, configure a coordinator node, then add worker nodes with `SELECT citus_add_node('worker_host', 5432)`. Create distributed tables with `SELECT create_distributed_table('events', 'tenant_id')`, specifying the shard key. Queries filtering by shard key execute on relevant workers only, providing linear scalability.

**Shard key selection critically impacts performance**. Choose high-cardinality columns appearing in WHERE clauses and JOIN conditions. Multi-tenant SaaS applications shard by tenant_id, IoT systems by device_id. Avoid sequential IDs, low-cardinality columns, or frequently updated values. Colocate related tables using the same shard key to enable efficient JOINs within workers: `SELECT create_distributed_table('users', 'tenant_id', colocate_with => 'events')`.

Schema-based sharding in Citus 12+ simplifies multi-tenant architectures. Each tenant gets a dedicated schema: `CREATE SCHEMA tenant_1`. Tables created in distributed schemas automatically shard by schema. This enables tenant isolation, per-tenant backups and straightforward migrations. Set `search_path TO tenant_1` to route queries to specific tenants.

## Search Capabilities

### Full-Text Search

PostgreSQL's built-in full-text search provides text search capabilities competitive with Elasticsearch for datasets under 5 million records. 

There are two complementary search modes:

- Tokenized full-text search (tsvector/tsquery): language-aware matching with ranking based on term frequency and normalization. PostgreSQL provides TF‑IDF like ranking via `ts_rank` and cover-density via `ts_rank_cd`. This is not strict BM25, though it behaves similarly for many use cases. For exact BM25 scoring, consider extensions (e.g., RUM or PGroonga), otherwise `ts_rank` normalization options are typically sufficient.

- Substring/fuzzy search (pg_trgm): fast substring, prefix and typo-tolerant matching using trigrams. This does not tokenize or stem; it matches character n-grams and is ideal for autocomplete, partial matches (`%term%`) and fuzzy lookups.

#### Token Search (TF-IDF)

Create tsvector columns using generated columns (PostgreSQL 12+) for automatic maintenance: `search_vector tsvector GENERATED ALWAYS AS (to_tsvector('english', coalesce(title, '') || ' ' || coalesce(body, ''))) STORED`. This pre-computes normalized lexemes, avoiding expensive on-the-fly computation during queries. Weight different fields using `setweight()` to prioritize titles over body text:

```sql
search_vector tsvector GENERATED ALWAYS AS (
    setweight(to_tsvector('english', coalesce(title, '')), 'A') ||
    setweight(to_tsvector('english', coalesce(body, '')), 'D')
) STORED
```

**GIN on tsvector** provides optimal full-text search performance, delivering **query speedups of 50-100x** compared to sequential scans. GIN indexes store an inverted index mapping each lexeme to matching document locations. Create with `CREATE INDEX idx_search ON documents USING GIN(search_vector)`. For read-heavy workloads, disable fastupdate: `CREATE INDEX idx_search ON documents USING GIN(search_vector) WITH (fastupdate = off)`. This trades slower writes for faster reads by eliminating the pending list that batches updates.

GiST indexes offer an alternative for write-heavy workloads. GiST builds tree structures with fixed-size document signatures, consuming less disk space and updating faster than GIN. However, GiST queries run approximately 3x slower and produce false positives requiring row rechecks. Choose GIN for most applications; reserve GiST for scenarios with severe write contention and limited disk space.

Query using the `@@` match operator: `WHERE search_vector @@ to_tsquery('english', 'postgresql & database')`. Use `websearch_to_tsquery()` for user input—it handles quoted phrases, OR operators and minus prefixes safely: `websearch_to_tsquery('english', '"full text" search -index')` converts to proper tsquery syntax. Rank results with `ts_rank()` or `ts_rank_cd()` (cover density ranking considering term proximity):

```sql
SELECT title, ts_rank(search_vector, query) AS rank
FROM articles, to_tsquery('english', 'postgresql & performance') query
WHERE search_vector @@ query
ORDER BY rank DESC
LIMIT 20;
```

**Properly optimized PostgreSQL full-text search achieves 6-10ms query times on 1.5 million records**, competitive with Elasticsearch's 20ms on equivalent datasets. Keys to optimization include stored tsvector columns, GIN indexes with fastupdate disabled and appropriate weighting. Real-world implementations show fintech companies like Qonto migrating from Elasticsearch to PostgreSQL FTS to simplify their stack while maintaining similar performance.

Advanced features include phrase searches with `phraseto_tsquery()`, prefix matching with `:*` operators and proximity searches with distance operators. Highlight matching terms in results using `ts_headline()` with custom start/stop delimiters. Configure multiple language dictionaries per database to support multilingual content, switching configurations per query.

#### Substring / Fuzzy (trigram)

Use pg_trgm when you need partial matches (`%term%`), autocomplete, or typo-tolerant search. It indexes character trigrams, not tokens, and works great alongside full-text search:

```sql
-- Enable extension once per database
CREATE EXTENSION IF NOT EXISTS pg_trgm;

-- GIN trigram index: best for filtering ILIKE/LIKE/% similarity
CREATE INDEX CONCURRENTLY idx_documents_title_trgm
  ON documents USING GIN (title gin_trgm_ops);

-- Optional: GiST trigram index if you need KNN ORDER BY <-> (nearest by similarity)
-- CREATE INDEX CONCURRENTLY idx_documents_title_trgm_gist
--   ON documents USING GIST (title gist_trgm_ops);
```

Common queries:

```sql
-- Substring match with ranking by similarity (works well with GIN)
SELECT id, title
FROM documents
WHERE title ILIKE '%postgres%'
ORDER BY similarity(title, 'postgres') DESC
LIMIT 20;

-- Fuzzy match using trigram similarity operator
SELECT id, title
FROM documents
WHERE title % 'postgras'                 -- allows typos
ORDER BY similarity(title, 'postgras') DESC
LIMIT 20;

-- Fast KNN if using GiST trigram index
SELECT id, title FROM documents ORDER BY title <-> 'postgres' LIMIT 20;

-- Tune threshold (default ~0.3) to control fuzziness
SELECT set_limit(0.4);  -- session-level
```

When to choose which:
- Use tsvector/tsquery (TF‑IDF-like ranking) for language-aware search, stemming, stop words and boolean/phrase queries.
- Use pg_trgm for substring/prefix matches, fuzzy lookup and autocomplete. It does not stem or understand language but excels at partial matches.
- Scaling: tsvector (TF‑IDF) generally scales better on large corpora (smaller, more selective indexes and lower write overhead). pg_trgm (trigram) indexes grow with text length and unique trigrams, can be much larger and heavier to maintain; prefer it for small/medium tables or short text fields (titles, usernames) and autocomplete.

### Vector Search

There are multiple powerful extensions for vector similarity search for Postgres - check out [this article](https://seanpedersen.github.io/posts/vector-databases) for an overview with benchmark.

### Geospatial Queries (PostGIS)

[PostGIS](https://postgis.net/) extends PostgreSQL with comprehensive geographic information system capabilities. Install with `CREATE EXTENSION postgis`, enabling spatial data types, measurement functions and specialized indexes. PostGIS 3.5+ requires PostgreSQL 12+ with GEOS 3.8+ and PROJ 6.1+ for full functionality. Recent versions introduced improvements like ST_CoverageClean for topology operations and enhanced SFCGAL support for 3D geometries.

PostGIS provides two core spatial types with distinct use cases. **Geometry types** perform planar calculations using Cartesian mathematics, appropriate for local or regional data within a few hundred kilometers. Geometry operations execute quickly but lose accuracy across large distances due to earth curvature. **Geography types** calculate on a spheroid representing Earth's actual shape, providing accurate great circle distances globally. Geography operations run slower but always return correct measurements in meters.

```sql
-- Geometry: fast but inaccurate for global distances
SELECT ST_Distance(
  'SRID=4326;POINT(-118.4 33.9)'::geometry,  -- LA
  'SRID=4326;POINT(2.5 49.0)'::geometry      -- Paris
);  -- Returns 122 degrees (meaningless)

-- Geography: accurate global distances  
SELECT ST_Distance(
  'SRID=4326;POINT(-118.4 33.9)'::geography,
  'SRID=4326;POINT(2.5 49.0)'::geography
);  -- Returns 9125170 meters (correct)
```

Spatial Reference System Identifiers (SRIDs) define coordinate systems. SRID 4326 represents WGS 84 (GPS coordinates), while 3857 provides Web Mercator used by mapping applications. Transform between projections with `ST_Transform(geom, target_srid)`. Always set SRIDs explicitly: `ST_SetSRID(ST_MakePoint(-73.9857, 40.7484), 4326)`. Operations require matching SRIDs or explicit transformation.

**GiST indexes provide 100-1000x query speedup** for spatial operations. Create with: `CREATE INDEX idx_places_geom ON places USING GIST (geom)`. For point datasets with uniform distribution, SP-GiST indexes build 3x faster and query 1.5x faster than GiST through quad-tree partitioning. BRIN indexes suit massive, spatially sorted datasets: index sizes are 2000x smaller than GiST (24KB versus 53MB for 1M points) but require sequential access patterns.

Proximity searches use `ST_DWithin()` for index-aware filtering: `WHERE ST_DWithin(location::geography, search_point::geography, 1000)` finds points within 1000 meters. Never use `WHERE ST_Distance() < 1000`—this prevents index usage and scans every row. For nearest neighbor queries, use the distance operator: `ORDER BY location <-> search_point LIMIT 5`. This leverages GiST index K-NN functionality for efficient ordering.

```sql
-- Find restaurants within 500m
SELECT name, ST_Distance(location::geography, search::geography) AS dist
FROM restaurants
CROSS JOIN (SELECT 'SRID=4326;POINT(-73.9857 40.7484)'::geography) s(search)
WHERE ST_DWithin(location::geography, s.search, 500)
ORDER BY dist;
```

Spatial relationship functions enable complex queries. `ST_Contains()` tests if one geometry fully encloses another, useful for point-in-polygon queries: `WHERE ST_Contains(neighborhood_boundary, business_location)`. `ST_Intersects()` determines overlap, `ST_Within()` tests containment and `ST_Crosses()` detects geometric crossings. Combine with aggregations for analytics: `SELECT neighborhood, COUNT(*) FROM businesses JOIN neighborhoods ON ST_Contains(boundary, location) GROUP BY neighborhood`.

Measurement functions calculate geometric properties. `ST_Area()` computes polygon areas (use geography for square meters, geometry for projected units). `ST_Length()` measures line lengths. `ST_Buffer()` creates zones around geometries: `ST_Buffer(location::geography, 500)` generates 500-meter radius circles around points. Subdivide complex geometries with `ST_Subdivide()` to improve query performance on large features.

Query optimization requires proper index usage and simplified geometries. Add bounding box pre-filters: `WHERE geom && ST_MakeEnvelope(xmin, ymin, xmax, ymax, 4326) AND ST_Intersects(geom, search_area)`. The `&&` operator performs fast bounding box overlap checks using the index before expensive geometric calculations. Use `ST_Centroid()` for approximate distance calculations on large polygons. Configure `random_page_cost = 1.0` for SSD storage and `effective_io_concurrency = 200` for NVMe.

## Embedded (Testing)

Check out [PGLite](https://pglite.dev/) for a small embeddable WebAssembly version of Postgres - being a good alternative to SQLite and thus ideal for testing and client-side deployments of Postgres (with the limitation of accepting only one connection though).

## Useful Commands

**Show sizes of all tables** + index + toast (The Oversized Attribute Storage Technique - used for long texts / json storage):

```sql
SELECT
   relname as table_name,
   pg_size_pretty(pg_total_relation_size(relid)) AS total_size,
   pg_size_pretty(pg_indexes_size(relid)) AS index_size,
   pg_size_pretty(pg_relation_size(relid)) AS data_size,
   pg_size_pretty(pg_table_size(relid) - pg_relation_size(relid)) AS toast_size
FROM pg_catalog.pg_statio_user_tables
ORDER BY pg_total_relation_size(relid) DESC;
```

**Show sizes of all indices for a table**:
```sql
SELECT
  i.indexrelname AS index_name,
  pg_size_pretty(pg_relation_size(i.indexrelid)) AS index_size,
  am.amname AS index_type
FROM
  pg_stat_all_indexes i
JOIN
  pg_class c ON i.indexrelid = c.oid
JOIN
  pg_am am ON c.relam = am.oid
WHERE
  i.relname = 'table_name'
ORDER BY
  pg_relation_size(i.indexrelid) DESC;
```

**Show sizes of all indices for a partitioned table**:
```sql
SELECT
  am.amname AS index_type,
  pg_size_pretty(SUM(pg_relation_size(i.indexrelid))) AS total_index_size
FROM
  pg_stat_all_indexes i
JOIN
  pg_class c ON i.indexrelid = c.oid
JOIN
  pg_am am ON c.relam = am.oid
JOIN
  pg_inherits pi ON pi.inhrelid = i.relid
WHERE
  pi.inhparent = 'table_name'::regclass
GROUP BY
  am.amname
ORDER BY
  SUM(pg_relation_size(i.indexrelid)) DESC;
```

**Show active transactions:**

```sql
SELECT pid, usename, datname, application_name, client_addr, backend_xid, xact_start, query, state
FROM pg_stat_activity
WHERE backend_xid IS NOT NULL;
```

## References

- <https://github.com/postgres/postgres>
- <https://www.postgresql.org/>
- <https://www.pixelstech.net/article/1747708863-openai%3a-scaling-postgresql-to-the-next-level>
  - [HN Discussion](https://news.ycombinator.com/item?id=44071418)

#programming
