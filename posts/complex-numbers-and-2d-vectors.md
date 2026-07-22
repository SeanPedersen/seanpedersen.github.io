---
date: '2026-07-22'
title: "Complex Numbers"
tags: [math]
---

  <p class="lead">A complex number is a 2D vector with an extra rule: it can multiply another point in the plane.
    That product turns multiplication into rotation and scaling. With conjugation, it also records the angle and signed
    area between two vectors.</p>

<style>
  .markdown-content {
    --complex-blue: #60a5fa;
    --complex-green: #34d399;
    --complex-orange: #fb923c;
    --complex-pink: #f472b6;
    --complex-grid: color-mix(in srgb, currentColor 16%, transparent);
    --complex-panel: color-mix(in srgb, currentColor 5%, transparent);
  }

  .markdown-content .lead {
    font-size: 1.15em;
  }

  .markdown-content .structure-table td,
  .markdown-content .structure-table th {
    vertical-align: top;
  }

  .markdown-content .diagram {
    background: var(--complex-panel);
    border: 0;
    display: block;
    height: auto;
    margin: 1.6rem auto 0.5rem;
    max-width: 46rem;
    overflow: hidden;
    position: relative;
    width: 100%;
    z-index: 0;
  }

  .markdown-content canvas.diagram {
    aspect-ratio: 2 / 1;
  }

  .markdown-content figcaption {
    font-size: 0.9em;
    opacity: 0.78;
    text-align: center;
  }

  .markdown-content figure {
    isolation: isolate;
    margin: 1.6rem 0;
    position: relative;
    z-index: 0;
  }

  .markdown-content .takeaway {
    border-left: 0.25rem solid var(--complex-blue);
    margin: 1.5rem 0;
    padding: 0.25rem 0 0.25rem 1rem;
  }

  .markdown-content .overline {
    text-decoration: overline;
  }

  .markdown-content .product-grid {
    table-layout: fixed;
  }

  .markdown-content .product-grid th,
  .markdown-content .product-grid td {
    text-align: center;
    vertical-align: middle;
  }

  .markdown-content .product-grid .real-term {
    background: color-mix(in srgb, var(--complex-blue) 14%, transparent);
  }

  .markdown-content .product-grid .imaginary-term {
    background: color-mix(in srgb, var(--complex-green) 14%, transparent);
  }

  .markdown-content .product-grid .term-result {
    display: block;
    font-size: 1.12em;
    font-weight: 700;
    margin-top: 0.25rem;
  }

  .markdown-content .glossary dt {
    font-weight: 700;
    margin-top: 0.9rem;
  }

  .markdown-content .glossary dd {
    margin: 0.2rem 0 0 1.25rem;
  }
</style>

  <p>This sounds like a small addition. It changes the kind of object we have. A <a
      href="https://en.wikipedia.org/wiki/Vector_space">2D real vector space</a> gives us addition and multiplication by
    real scalars. The <a href="https://en.wikipedia.org/wiki/Complex_number">complex numbers</a> also let us multiply and
    divide elements of the plane.</p>

  <p>Before adding complex multiplication, a 2D vector space already has a standard way to compare directions. The
    <a href="https://en.wikipedia.org/wiki/Dot_product">dot product</a> of u = (a, b) and v = (c, d) is:</p>

$$
u \cdot v = ac + bd = |u||v|\cos\theta
$$

  <p>It is positive when the vectors point in similar directions, zero when they are perpendicular, and negative when
    they point apart. Its value also grows with both vector lengths.</p>

  <p><a href="https://en.wikipedia.org/wiki/Cosine_similarity">Cosine similarity</a> removes those lengths. It keeps only
    directional alignment:</p>

$$
\frac{u \cdot v}{|u||v|} = \cos\theta
$$

  <p>This gives a value from -1 to 1. Complex multiplication will add the part these measures omit: which side one
    vector lies on, and how much signed area the pair spans.</p>

  <div class="table-wrapper">
    <table class="structure-table">
      <thead>
        <tr>
          <th>2D real vector space</th>
          <th>Complex numbers</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td>Elements look like (x, y)</td>
          <td>Elements look like a + bi</td>
        </tr>
        <tr>
          <td>Adds vectors component by component</td>
          <td>Adds numbers component by component</td>
        </tr>
        <tr>
          <td>Multiplies a vector by a real scalar</td>
          <td>Multiplies two complex numbers</td>
        </tr>
        <tr>
          <td>Has no required vector by vector product</td>
          <td>Forms a <a href="https://en.wikipedia.org/wiki/Field_(mathematics)">field</a></td>
        </tr>
      </tbody>
    </table>
  </div>

  <h2 id="same-plane">The same plane</h2>

  <p>The match between them is exact if we only look at addition and real scaling:</p>

$$a + bi \longleftrightarrow (a, b)$$

  <p>For example, 3 + 2i is the point (3, 2). Adding 1 + i moves one unit right and one unit up. As
    real vector spaces, ℂ and ℝ<sup>2</sup> are isomorphic. Each has two real directions.</p>

  <figure>
    <canvas class="diagram complex-canvas" data-diagram="plane" role="img"
      aria-label="The complex number 3 + 2i and vector (3, 2) shown at the same point in the plane."></canvas>
    <figcaption>One point, two readings. The difference appears when we ask what operations are allowed.</figcaption>
  </figure>

  <h2 id="complex-multiplication">Complex multiplication rotates and scales</h2>

  <p>A vector space does not tell us what (1, 2)(3, 4) means. We can add these vectors or scale them,
    but a product needs a new definition.</p>

  <p>Complex numbers come with that definition. Since i<sup>2</sup> = -1:</p>

$$
(a + bi)(c + di) = (ac - bd) + (ad + bc)i
$$

  <figure>
    <div class="table-wrapper">
      <table class="product-grid" aria-label="Four terms of complex multiplication">
        <thead><tr><th>×</th><th>c</th><th>di</th></tr></thead>
        <tbody>
          <tr>
            <th>a</th>
            <td class="real-term">a × c<span class="term-result">ac</span></td>
            <td class="imaginary-term">a × di<span class="term-result">adi</span></td>
          </tr>
          <tr>
            <th>bi</th>
            <td class="imaginary-term">bi × c<span class="term-result">bci</span></td>
            <td class="real-term">bi × di<span class="term-result">bd i<sup>2</sup> = -bd</span></td>
          </tr>
        </tbody>
      </table>
    </div>
    <figcaption>Blue cells contribute to the real part. Green cells contribute to the imaginary part.</figcaption>
  </figure>

  <p>Notice that ac - bd is <strong>not</strong> the dot product. The minus sign comes from multiplying the two
    imaginary terms:</p>

$$
(bi)(di) = bd\,i^2 = -bd
$$

  <p>The dot product ac + bd appears only when we conjugate one input first. Conjugation changes bi to -bi, so the
    imaginary terms contribute +bd instead. The later comparison section uses exactly this sign change.</p>

  <p>This rule works with addition and gives every nonzero number a reciprocal. It is not part of a
    plain vector space. We chose extra algebraic structure.</p>

  <p>The structure has a clear geometric meaning. Write a nonzero complex number in
    <a href="https://en.wikipedia.org/wiki/Complex_number#Polar_form">polar form</a>:</p>

$$
z = r(\cos \theta + i\sin \theta) = re^{i\theta}
$$

  <p>The bridge between the trigonometric and exponential forms is
    <a href="https://en.wikipedia.org/wiki/Euler%27s_formula">Euler's formula</a>:</p>

$$
e^{i\theta} = \cos\theta + i\sin\theta
$$

  <p>When |z| = 1, we have r = 1 and may write z = e<sup>iθ</sup>. For a general complex number, the factor r is still
    needed. Setting θ = π gives Euler's identity:</p>

$$
e^{i\pi} + 1 = 0
$$

  <p>Multiplication by z scales every length by r and rotates every angle by θ. If r = 1, it only
    rotates. Multiplying by i rotates 90 degrees counterclockwise.</p>

  <figure>
    <canvas class="diagram complex-canvas" data-diagram="multiplication" role="img"
      aria-label="The vectors w, z, and zw showing that multiplication by z rotates w by theta and scales its length by the modulus of z."></canvas>
    <figcaption>Here w = z<sub>1</sub> and z = z<sub>2</sub>. Normal multiplication produces
      wz = z<sub>1</sub>z<sub>2</sub> by adding their angles.</figcaption>
  </figure>

  <h2 id="compare-vectors">Conjugation turns multiplication into comparison</h2>

  <p>Normal complex multiplication combines two rotations and scales. If z<sub>1</sub> has angle θ<sub>1</sub> and
    z<sub>2</sub> has angle θ<sub>2</sub>, their product has angle θ<sub>1</sub> + θ<sub>2</sub>. This is useful when one
    complex number acts as a transformation on another:</p>

$$
z_1z_2 = |z_1||z_2|e^{i(\theta_1+\theta_2)}
$$

  <p>Sometimes we do not want to combine the vectors. We want to compare them. Conjugating z<sub>1</sub> reverses its
    angle from θ<sub>1</sub> to -θ<sub>1</sub>. Multiplication then subtracts the first angle from the second:</p>

$$
\overline{z_1}z_2 = |z_1||z_2|e^{i(\theta_2-\theta_1)}
$$

  <p>The result now describes the direction of z<sub>2</sub> relative to z<sub>1</sub>. If both vectors rotate by the same
    angle, this result does not change. Use normal multiplication to apply or combine rotations. Use the conjugated
    product to measure relative angle, alignment, orientation, or signed area.</p>

  <p>Let z<sub>1</sub> = a + bi and z<sub>2</sub> = c + di. Take the
    <a href="https://en.wikipedia.org/wiki/Complex_conjugate">complex conjugate</a> of z<sub>1</sub> first, then multiply
    it by z<sub>2</sub>:</p>

$$
\overline{z_1}z_2 = (a-bi)(c+di) = (ac+bd) + (ad-bc)i
$$

  <p>This is different from the ordinary product z<sub>1</sub>z<sub>2</sub>, whose real part is ac - bd. A
    complex conjugate flips the sign of the imaginary part. It is not the multiplicative inverse. The inverse also
    divides by the squared modulus:</p>

$$
{z_1}^{-1} = \frac{\overline{z_1}}{|z_1|^2}
$$

  <p>The real part is the dot product. It
    measures alignment:</p>

$$
ac + bd = |z_1||z_2|\cos\Delta\theta
$$

  <p>The imaginary part is the <a href="https://en.wikipedia.org/wiki/Determinant">determinant</a>.
    It measures signed area and orientation:</p>

$$
ad - bc = |z_1||z_2|\sin\Delta\theta
$$

  <p>Here Δθ = θ<sub>2</sub> - θ<sub>1</sub>. A positive imaginary part means z<sub>2</sub> lies
    counterclockwise from z<sub>1</sub>, using this order. Swapping the inputs flips the sign.</p>

  <figure>
    <canvas class="diagram complex-canvas" data-diagram="comparison" role="img"
      aria-label="Vectors z1 and z2 with their angle, projection, signed parallelogram area, and conjugated product z3."></canvas>
    <figcaption>The dot product becomes the horizontal coordinate of z<sub>3</sub>. The signed area becomes its vertical
      coordinate. Together they form z<sub>3</sub> = <span class="overline">z<sub>1</sub></span>z<sub>2</sub>.</figcaption>
  </figure>

  <h2 id="what-is-preserved">Encoded, not conserved</h2>

  <p>It is tempting to say that the conjugate product conserves angle and area. It does not preserve
    them through a change. It <strong>encodes</strong> the relative angle and combined scale of the
    original pair.</p>

  <p>If z<sub>1</sub> = r<sub>1</sub>e<sup>iθ₁</sup> and z<sub>2</sub> =
    r<sub>2</sub>e<sup>iθ₂</sup>, then Euler's formula gives:</p>

$$
\overline{z_1}z_2 = r_1r_2e^{i(\theta_2-\theta_1)}
$$

  <p>Its magnitude is r<sub>1</sub>r<sub>2</sub>. Its argument is the signed angle from
    z<sub>1</sub> to z<sub>2</sub>. Its real and imaginary parts split that same information into
    alignment and area.</p>

  <div class="takeaway">
    <p><strong>The short version:</strong> ℝ<sup>2</sup> and ℂ are the same real vector space, but
      not the same algebraic object. Complex multiplication turns a point into a rotate-and-scale
      operation. With conjugation, it also packs the dot product and signed area into one number.</p>
  </div>

  <h2 id="outlook">Outlook: why this trick stops at the plane</h2>

  <p>One complex number has exactly two real coordinates. This is why its geometry fits the 2D plane so well. The rule
    i<sup>2</sup> = -1 supplies one fixed quarter-turn, and complex multiplication builds every rotation and scaling from
    it.</p>

  <p>Sadly, there is no matching number system for every ℝ<sup>N</sup> that keeps all the useful properties of ℂ. We
    cannot give an arbitrary real vector space a multiplication that is commutative, associative, distributive, and
    divisible by every nonzero element.</p>

  <p>Even-dimensional spaces still have a partial extension. After choosing a
    <a href="https://encyclopediaofmath.org/wiki/Complex_structure">complex structure</a>, ℝ<sup>2n</sup> can be viewed as
    ℂ<sup>n</sup>:</p>

$$
\mathbb{R}^{2n} \cong \mathbb{C}^{n}
$$

  <p>This choice pairs real directions and defines an operator J with J<sup>2</sup> = -I. It lets complex scalars act on
    the space. It does not turn the whole space into one large complex number field. In particular, odd-dimensional real
    spaces cannot carry such a complex structure because their directions cannot all be paired.</p>

  <p>Dimension four offers another clue. The <a href="https://en.wikipedia.org/wiki/Quaternion">quaternions</a> give
    ℝ<sup>4</sup> a useful multiplication and division, but multiplication is no longer commutative. The
    <a href="https://encyclopediaofmath.org/wiki/Frobenius_theorem">Frobenius theorem</a> makes the limit precise: ℝ and
    ℂ are the only finite-dimensional associative, commutative real division algebras. To move beyond the complex plane,
    some familiar algebraic rule has to go.</p>

  <h2 id="glossary">Glossary</h2>

  <dl class="glossary">
    <dt>Vector space</dt>
    <dd>A set whose elements can be added together and multiplied by scalars while following the vector space rules.</dd>
    <dt>Scalar</dt>
    <dd>A number used to scale a vector. The scalars in a 2D real vector space are real numbers.</dd>
    <dt>Field</dt>
    <dd>A number system with addition, subtraction, multiplication, and division by every nonzero element. Both ℝ and ℂ
      are fields.</dd>
    <dt>Complex conjugate</dt>
    <dd>The reflection of a complex number across the real axis. The conjugate of z = a + bi is
      <span class="overline">z</span> = a - bi.</dd>
    <dt>Modulus</dt>
    <dd>The length of a complex number viewed as a vector: |z| = √(a<sup>2</sup> + b<sup>2</sup>).</dd>
    <dt>Argument</dt>
    <dd>The signed angle of a nonzero complex number from the positive real axis, often written arg(z).</dd>
    <dt>Dot product</dt>
    <dd>A length-weighted measure of alignment. For u = (a, b) and v = (c, d), it equals ac + bd.</dd>
    <dt>Cosine similarity</dt>
    <dd>The dot product divided by both vector lengths. It measures directional alignment from -1 to 1.</dd>
    <dt>Determinant</dt>
    <dd>For two 2D vectors, the signed area of their parallelogram. Its sign records their orientation.</dd>
    <dt>Polar form</dt>
    <dd>A complex number written through its length and angle: z = re<sup>iθ</sup>.</dd>
    <dt>Conjugated product</dt>
    <dd>The product <span class="overline">z<sub>1</sub></span>z<sub>2</sub>. Its real part is the dot product, and its
      imaginary part is the signed area.</dd>
  </dl>

  <h2 id="references">References</h2>

  <ol>
    <li><a href="https://encyclopediaofmath.org/wiki/Complex_number">Encyclopedia of Mathematics: Complex number</a></li>
    <li><a href="https://mathworld.wolfram.com/ComplexMultiplication.html">MathWorld: Complex Multiplication</a></li>
    <li><a href="https://mathworld.wolfram.com/Rotation.html">MathWorld: Rotation</a></li>
    <li><a href="https://encyclopediaofmath.org/wiki/Complex_structure">Encyclopedia of Mathematics: Complex structure</a></li>
    <li><a href="https://encyclopediaofmath.org/wiki/Frobenius_theorem">Encyclopedia of Mathematics: Frobenius theorem</a></li>
    <li><a href="https://youtu.be/f079K1f2WQk">YouTube: Necessity of complex numbers</a></li>
  </ol>

<script>
  /** Responsive mathematical canvas diagrams for the complex numbers post. */
  (() => {
    const LOGICAL_WIDTH = 700;
    const LOGICAL_HEIGHT = 350;
    const COLORS = {
      blue: "#60a5fa",
      green: "#34d399",
      orange: "#fb923c",
      pink: "#f472b6",
      purple: "#a78bfa",
    };
    const EXAMPLE_VECTORS = {
      first: { x: 2.8, y: 0.6 },
      second: { x: 1.2, y: 2.2 },
    };

    const add = (left, right) => ({ x: left.x + right.x, y: left.y + right.y });
    const scale = (vector, factor) => ({ x: vector.x * factor, y: vector.y * factor });
    const dot = (left, right) => left.x * right.x + left.y * right.y;
    const magnitude = (vector) => Math.hypot(vector.x, vector.y);
    const fromPolar = (length, angle) => ({ x: length * Math.cos(angle), y: length * Math.sin(angle) });
    const complexMultiply = (left, right) => ({
      x: left.x * right.x - left.y * right.y,
      y: left.x * right.y + left.y * right.x,
    });
    const conjugate = (vector) => ({ x: vector.x, y: -vector.y });

    const toCanvasPoint = (origin, unit, vector) => ({
      x: origin.x + vector.x * unit,
      y: origin.y - vector.y * unit,
    });

    const drawLine = (context, start, end, color, width = 2, dash = []) => {
      context.beginPath();
      context.setLineDash(dash);
      context.moveTo(start.x, start.y);
      context.lineTo(end.x, end.y);
      context.strokeStyle = color;
      context.lineWidth = width;
      context.stroke();
      context.setLineDash([]);
    };

    const drawVector = (context, origin, end, color) => {
      const angle = Math.atan2(end.y - origin.y, end.x - origin.x);
      const arrowLength = 13;
      const arrowAngle = Math.PI / 7;

      drawLine(context, origin, end, color, 4);
      context.beginPath();
      context.moveTo(end.x, end.y);
      context.lineTo(
        end.x - arrowLength * Math.cos(angle - arrowAngle),
        end.y - arrowLength * Math.sin(angle - arrowAngle),
      );
      context.lineTo(
        end.x - arrowLength * Math.cos(angle + arrowAngle),
        end.y - arrowLength * Math.sin(angle + arrowAngle),
      );
      context.closePath();
      context.fillStyle = color;
      context.fill();
    };

    const drawLabel = (context, text, point, color, align = "left") => {
      context.fillStyle = color;
      context.font = "16px system-ui, sans-serif";
      context.textAlign = align;
      context.textBaseline = "middle";
      context.fillText(text, point.x, point.y);
    };

    const drawAxes = (context, origin, foreground) => {
      context.globalAlpha = 0.5;
      drawLine(context, { x: 45, y: origin.y }, { x: 665, y: origin.y }, foreground);
      drawLine(context, { x: origin.x, y: 25 }, { x: origin.x, y: 325 }, foreground);
      context.globalAlpha = 1;
      drawLabel(context, "Real", { x: 625, y: origin.y + 22 }, foreground);
      drawLabel(context, "Imaginary", { x: origin.x - 12, y: 38 }, foreground, "right");
    };

    const drawAxisNumbers = (context, origin, unit, maximum, foreground) => {
      context.fillStyle = foreground;
      context.font = "12px system-ui, sans-serif";
      context.globalAlpha = 0.72;

      for (let value = 1; value <= maximum; value += 1) {
        const x = origin.x + value * unit;
        const y = origin.y - value * unit;
        drawLine(context, { x, y: origin.y - 4 }, { x, y: origin.y + 4 }, foreground, 1);
        drawLine(context, { x: origin.x - 4, y }, { x: origin.x + 4, y }, foreground, 1);
        context.textAlign = "center";
        context.textBaseline = "top";
        context.fillText(String(value), x, origin.y + 7);
        context.textAlign = "right";
        context.textBaseline = "middle";
        context.fillText(String(value), origin.x - 9, y);
      }

      context.globalAlpha = 1;
    };

    const drawAngle = (context, origin, radius, startAngle, endAngle, color, label) => {
      context.beginPath();
      context.arc(origin.x, origin.y, radius, -startAngle, -endAngle, true);
      context.strokeStyle = color;
      context.lineWidth = 2;
      context.stroke();

      const middleAngle = (startAngle + endAngle) / 2;
      const labelPoint = toCanvasPoint(origin, radius * 0.68, fromPolar(1, middleAngle));
      drawLabel(context, label, labelPoint, color, "center");
    };

    const drawPlaneDiagram = (context, foreground) => {
      const origin = { x: 110, y: 280 };
      const unit = 70;
      const value = { x: 3, y: 2 };
      const end = toCanvasPoint(origin, unit, value);

      context.globalAlpha = 0.13;
      for (let x = 1; x <= 7; x += 1) {
        drawLine(context, { x: origin.x + x * unit, y: 25 }, { x: origin.x + x * unit, y: 325 }, foreground);
      }
      for (let y = -0.5; y <= 3.5; y += 1) {
        const canvasY = origin.y - y * unit;
        drawLine(context, { x: 45, y: canvasY }, { x: 665, y: canvasY }, foreground);
      }
      context.globalAlpha = 1;

      drawAxes(context, origin, foreground);
      drawLine(context, toCanvasPoint(origin, unit, { x: value.x, y: 0 }), end, COLORS.green, 2, [7, 6]);
      drawLine(context, toCanvasPoint(origin, unit, { x: 0, y: value.y }), end, COLORS.green, 2, [7, 6]);
      drawVector(context, origin, end, COLORS.blue);
      drawLabel(context, "3 + 2i", { x: end.x + 18, y: end.y - 12 }, foreground);
      drawLabel(context, "vector (3, 2)", { x: end.x + 18, y: end.y + 14 }, foreground);
    };

    const drawMultiplicationDiagram = (context, foreground) => {
      const origin = { x: 105, y: 310 };
      const unit = 37;
      const w = EXAMPLE_VECTORS.first;
      const z = EXAMPLE_VECTORS.second;
      const product = complexMultiply(w, z);
      const wAngle = Math.atan2(w.y, w.x);
      const theta = Math.atan2(z.y, z.x);
      const wEnd = toCanvasPoint(origin, unit, w);
      const zEnd = toCanvasPoint(origin, unit, z);
      const productEnd = toCanvasPoint(origin, unit, product);

      drawAxes(context, origin, foreground);
      drawAxisNumbers(context, origin, unit, 7, foreground);
      drawAngle(context, origin, 52, wAngle, wAngle + theta, COLORS.green, "θ₂");
      drawVector(context, origin, wEnd, COLORS.blue);
      drawVector(context, origin, zEnd, COLORS.green);
      drawVector(context, origin, productEnd, COLORS.orange);
      drawLabel(context, `w = z₁ = (${w.x}, ${w.y})`, { x: wEnd.x + 10, y: wEnd.y - 15 }, COLORS.blue);
      drawLabel(context, `z = z₂ = (${z.x}, ${z.y})`, { x: zEnd.x + 10, y: zEnd.y - 13 }, COLORS.green);
      drawLabel(context, `wz = z₁z₂ = (${product.x.toFixed(2)}, ${product.y.toFixed(2)})`,
        { x: productEnd.x + 12, y: productEnd.y + 10 }, COLORS.orange);
      drawLabel(context, "length × |z₂|", { x: productEnd.x + 30, y: 170 }, foreground);
    };

    const drawComparisonDiagram = (context, foreground) => {
      const origin = { x: 105, y: 290 };
      const unit = 45;
      const first = EXAMPLE_VECTORS.first;
      const second = EXAMPLE_VECTORS.second;
      const firstEnd = toCanvasPoint(origin, unit, first);
      const secondEnd = toCanvasPoint(origin, unit, second);
      const sumEnd = toCanvasPoint(origin, unit, add(first, second));
      const product = complexMultiply(conjugate(first), second);
      const productEnd = toCanvasPoint(origin, unit, product);
      const productRealEnd = toCanvasPoint(origin, unit, { x: product.x, y: 0 });
      const productImaginaryEnd = toCanvasPoint(origin, unit, { x: 0, y: product.y });
      const projection = scale(first, dot(first, second) / (magnitude(first) ** 2));
      const projectionEnd = toCanvasPoint(origin, unit, projection);
      const firstAngle = Math.atan2(first.y, first.x);
      const secondAngle = Math.atan2(second.y, second.x);

      drawAxes(context, origin, foreground);
      drawAxisNumbers(context, origin, unit, 5, foreground);
      context.beginPath();
      context.moveTo(origin.x, origin.y);
      context.lineTo(firstEnd.x, firstEnd.y);
      context.lineTo(sumEnd.x, sumEnd.y);
      context.lineTo(secondEnd.x, secondEnd.y);
      context.closePath();
      context.fillStyle = "rgb(52 211 153 / 18%)";
      context.fill();
      drawLine(context, firstEnd, sumEnd, COLORS.green, 2, [7, 6]);
      drawLine(context, secondEnd, sumEnd, COLORS.green, 2, [7, 6]);
      drawLine(context, origin, projectionEnd, COLORS.orange, 7);
      drawAngle(context, origin, 66, firstAngle, secondAngle, foreground, "Δθ");
      drawLine(context, productEnd, productRealEnd, COLORS.purple, 2, [7, 6]);
      drawLine(context, productEnd, productImaginaryEnd, COLORS.purple, 2, [7, 6]);
      drawVector(context, origin, firstEnd, COLORS.blue);
      drawVector(context, origin, secondEnd, COLORS.pink);
      drawVector(context, origin, productEnd, COLORS.purple);
      drawLabel(context, `z₁ = (${first.x}, ${first.y})`, { x: firstEnd.x + 10, y: firstEnd.y - 15 }, COLORS.blue);
      drawLabel(context, `z₂ = (${second.x}, ${second.y})`, { x: secondEnd.x + 10, y: secondEnd.y - 13 }, COLORS.pink);
      drawLabel(context, `z₃ = (${product.x.toFixed(2)}, ${product.y.toFixed(2)})`, { x: productEnd.x + 12, y: productEnd.y + 12 }, COLORS.purple);
      drawLabel(context, `Re(z₃) = ${product.x.toFixed(2)}`, { x: productRealEnd.x, y: origin.y + 42 }, COLORS.purple, "center");
      drawLabel(context, `Im(z₃) = ${product.y.toFixed(2)}`, { x: origin.x + 12, y: productImaginaryEnd.y + 16 }, COLORS.purple);
      drawLabel(context, "signed area = Im", scale(add(firstEnd, secondEnd), 0.5), COLORS.green, "center");
      drawLabel(context, "alignment = Re", { x: projectionEnd.x, y: origin.y + 42 }, COLORS.orange, "center");
    };

    const DRAWERS = {
      plane: drawPlaneDiagram,
      multiplication: drawMultiplicationDiagram,
      comparison: drawComparisonDiagram,
    };

    const renderCanvas = (canvas) => {
      const context = canvas.getContext("2d");
      const bounds = canvas.getBoundingClientRect();
      const pixelRatio = Math.min(window.devicePixelRatio || 1, 2);
      const logicalScale = bounds.width / LOGICAL_WIDTH;
      canvas.width = Math.round(bounds.width * pixelRatio);
      canvas.height = Math.round(bounds.width * LOGICAL_HEIGHT / LOGICAL_WIDTH * pixelRatio);
      context.setTransform(pixelRatio * logicalScale, 0, 0, pixelRatio * logicalScale, 0, 0);
      context.clearRect(0, 0, LOGICAL_WIDTH, LOGICAL_HEIGHT);
      context.lineCap = "round";
      context.lineJoin = "round";

      const foreground = getComputedStyle(canvas).color;
      DRAWERS[canvas.dataset.diagram]?.(context, foreground);
    };

    const canvases = [...document.querySelectorAll("canvas[data-diagram]")];
    const renderAll = () => canvases.forEach(renderCanvas);
    const resizeObserver = new ResizeObserver(renderAll);
    const themeObserver = new MutationObserver(renderAll);

    canvases.forEach((canvas) => resizeObserver.observe(canvas));
    themeObserver.observe(document.documentElement, { attributes: true, attributeFilter: ["data-theme"] });
    renderAll();
  })();
</script>
