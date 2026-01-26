(function () {
  'use strict';

  // Matrix easter egg module
  const Matrix = {
    canvas: null,
    ctx: null,
    animationFrame: null,
    yPositions: [],
    lastTime: 0,
    activePhrase: null,
    nextPhraseTimeMilliSecs: 0,
    isActive: false,

    // Configuration
    config: {
      matrixFrequency: 12.69, // update frequency in hertz
      matrixTerminator: 0.23, // symbol drawing probability
      symbolSize: 23,
      phrases: ["WAKE UP NEO", "FREE YOUR MIND", "FOLLOW THE WHITE RABBIT"],
      phraseIntervalSecs: 23,
      // Matrix character set
      matrixChars: "アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲン0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz∀∁∂∃∄∅∆∇∈∉∊∋∌∍∎∏∐∑−∓∔∕∖∗∘∙√∛∜∝∞∟∠∡∢∣∤∥∦∧∨∩∪∫∬∭∮∯∰∱∲∳∴∵∶∷∸∹∺∻∼∽∾∿≀≁≂≃≄≅≆≇≈≉≊≋≌≍≎≏≐≑≒≓≔≕≖≗≘≙≚≛≜≝≞≟≠≡≢≣≤≥≦≧≨≩≪≫≬≭≮≯≰≱≲≳≴≵≶≷≸≹≺≻≼≽≾≿⊀⊁⊂⊃⊄⊅⊆⊇⊈⊉⊊⊋⊌⊍⊎⊏⊐⊑⊒⊓⊔⊕⊖⊗⊘⊙⊚⊛⊜⊝⊞⊟⊠⊡⊢⊣⊤⊥⊦⊧⊨⊩⊪⊫⊬⊭⊮⊯⊰⊱⊲⊳⊴⊵⊶⊷⊸⊹⊺⊻⊼⊽⊾⊿⋀⋁⋂⋃⋄⋅⋆⋇⋈⋉⋊⋋⋌⋍⋎⋏⋐⋑⋒⋓⋔⋕⋖⋗⋘⋙⋚⋛⋜⋝⋞⋟⋠⋡⋢⋣⋤⋥⋦⋧⋨⋩⋪⋫⋬⋭⋮⋯⋰⋱⋲⋳⋴⋵⋶⋷⋸⋹⋺⋻⋼⋽⋾⋿░▔▮▯▰▱▲△▴▵▶▷▻▼▽▾▿◀◁◅◆◇◈◉◊○◌◍◎●◐◑◒◓◔◕◖◗◘◙◜◝◞◟◠◡◢◣◤◥◦◧◨◩◪◫◬◭◮◯☆☇☈☉☓☜☝☞☟☡☤☥☦☧☨☩☮☯☰☱☲☳☴☵☶☷"
    },

    init(width, height) {
      if (this.isActive) return;

      // Create container wrapper
      const wrapper = document.createElement('div');
      wrapper.id = 'matrixWrapper';
      wrapper.style.position = 'fixed';
      wrapper.style.top = '0';
      wrapper.style.left = '0';
      wrapper.style.width = '100%';
      wrapper.style.height = '100%';
      wrapper.style.zIndex = '-1'; // Behind all UI elements
      wrapper.style.pointerEvents = 'none';
      wrapper.style.overflow = 'hidden';

      // Create canvas element
      this.canvas = document.createElement('canvas');
      this.canvas.id = 'matrixCanvas';
      this.canvas.style.position = 'absolute';
      this.canvas.style.top = '0';
      this.canvas.style.left = '0';
      this.canvas.style.width = '100%';
      this.canvas.style.height = '100%';
      this.canvas.style.opacity = '0.5'; // Semi-transparent so content is readable
      this.canvas.style.pointerEvents = 'none';

      this.canvas.width = width;
      this.canvas.height = height;

      this.ctx = this.canvas.getContext('2d');
      if (!this.ctx) return;

      // Set initial variables
      const intervalMs = Math.random() * 3.6 + 1000 * (1 / this.config.matrixFrequency);
      this.intervalMs = intervalMs;

      const nColumns = Math.floor(width / this.config.symbolSize);
      this.nColumns = nColumns;
      this.yPositions = Array(nColumns).fill(0);

      // Initial transparent background (no black fill needed)
      this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);

      // Add canvas to wrapper, wrapper to DOM
      wrapper.appendChild(this.canvas);
      document.body.appendChild(wrapper);
      this.wrapper = wrapper;

      this.isActive = true;
      this.lastTime = 0;
      this.nextPhraseTimeMilliSecs = 0;

      // Start animation
      this.animationFrame = requestAnimationFrame(this.animate.bind(this));
    },

    animate(currentTimeMilliSecs) {
      if (!this.isActive) return;

      if (this.lastTime === 0) {
        this.lastTime = currentTimeMilliSecs;
        this.nextPhraseTimeMilliSecs = currentTimeMilliSecs + Math.random() * 10000;
      }

      // Throttle based on frequency
      if (currentTimeMilliSecs - this.lastTime < this.intervalMs) {
        this.animationFrame = requestAnimationFrame(this.animate.bind(this));
        return;
      }
      this.lastTime = currentTimeMilliSecs;

      // Adapt matrix size to screen
      const newColumns = Math.floor(this.canvas.width / this.config.symbolSize);
      if (this.nColumns !== newColumns) {
        this.nColumns = newColumns;
        this.yPositions = Array(newColumns).fill(0);
      }

      // Check if it's time for a new phrase
      if (currentTimeMilliSecs > this.nextPhraseTimeMilliSecs && !this.activePhrase) {
        const randomPhrase = this.config.phrases[Math.floor(Math.random() * this.config.phrases.length)];
        const randomColumn = Math.floor(Math.random() * this.nColumns);
        console.log(`New phrase: "${randomPhrase}" at column ${randomColumn}`);
        this.activePhrase = {
          phrase: randomPhrase,
          columnIndex: randomColumn,
          charIndex: 0,
        };
        this.nextPhraseTimeMilliSecs = currentTimeMilliSecs + this.config.phraseIntervalSecs * 1000 + (Math.random() - 0.5) * 10000;
      }

      // Set matrix style with fade effect
      this.ctx.fillStyle = 'rgba(0, 0, 0, 0.05)';
      this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);
      this.ctx.fillStyle = '#0f0';
      this.ctx.font = `${this.config.symbolSize}px "Courier New", monospace`;

      // Update matrix symbols
      this.yPositions.forEach((y, index) => {
        let text = null;
        let shouldDraw = true;

        if (this.activePhrase && this.activePhrase.columnIndex === index) {
          const { phrase, charIndex } = this.activePhrase;
          if (charIndex < phrase.length) {
            text = phrase[charIndex];
            this.activePhrase.charIndex++;
          } else {
            this.activePhrase = null;
            // Fallback to random char
            const randomIndex = Math.floor(Math.random() * this.config.matrixChars.length);
            text = this.config.matrixChars[randomIndex];
          }
        } else {
          if (this.config.matrixTerminator > Math.random()) {
            const randomIndex = Math.floor(Math.random() * this.config.matrixChars.length);
            text = this.config.matrixChars[randomIndex];
          } else {
            shouldDraw = false;
          }
        }

        if (shouldDraw && text) {
          const x = index * this.config.symbolSize;
          this.ctx.fillText(text, x, y);

          this.yPositions[index] = y + this.config.symbolSize;

          // Reset position when off screen, but give extra space for phrases
          if (y > this.canvas.height + this.config.symbolSize * 2) {
            this.yPositions[index] = 0;
          }
        }
      });

      this.animationFrame = requestAnimationFrame(this.animate.bind(this));
    },

    destroy() {
      if (!this.isActive) return;

      this.isActive = false;

      if (this.animationFrame) {
        cancelAnimationFrame(this.animationFrame);
        this.animationFrame = null;
      }

      if (this.wrapper && this.wrapper.parentNode) {
        this.wrapper.parentNode.removeChild(this.wrapper);
      }

      this.wrapper = null;
      this.canvas = null;
      this.ctx = null;
      this.yPositions = [];
      this.lastTime = 0;
      this.activePhrase = null;
      this.nextPhraseTimeMilliSecs = 0;
    }
  };

  // Export to global scope
  window.Matrix = Matrix;
})();
