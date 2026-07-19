// draw.js: mdbook-draw canvas initializer
// mdBook injects additional-js scripts at the bottom of <body>, so the DOM
// is fully parsed by the time this runs. No event listener needed.

(function () {
  var STORAGE_PREFIX = "mdbook-draw:";

  var toolbars = document.querySelectorAll(".mdbook-draw-toolbar");

  toolbars.forEach(function (toolbar) {
    var canvasId = toolbar.getAttribute("data-canvas-id");
    var canvas = document.getElementById(canvasId);

    if (!canvas) {
      console.warn("mdbook-draw: canvas not found for id:", canvasId);
      return;
    }

    var ctx = canvas.getContext("2d");
    var drawing = false;
    var startPos = null;
    var snapshot = null; // ImageData taken at mousedown
    var tool = "pencil";
    var color = "#000000";
    var brushSize = 4;
    var background = canvas.getAttribute("data-background") || "#ffffff";
    var storageKey = STORAGE_PREFIX + canvasId;
    var bgKey = STORAGE_PREFIX + canvasId + ":bg";

    // --- Persistence helpers ---
    function isShapeTool(t) {
      return t === "line" || t === "circle";
    }

    function applyStroke() {
      ctx.strokeStyle = tool === "eraser" ? background : color;
      ctx.lineWidth = tool === "eraser" ? brushSize * 3 : brushSize;
      ctx.lineCap = "round";
      ctx.lineJoin = "round";
    }

    function drawShape(from, to, shiftHeld) {
      applyStroke();
      ctx.beginPath();
      if (tool === "line") {
        ctx.moveTo(from.x, from.y);
        ctx.lineTo(to.x, to.y);
      } else {
        var dx = to.x - from.x;
        var dy = to.y - from.y;
        if (shiftHeld) {
          // circle from center, radius = drag distance
          ctx.arc(from.x, from.y, Math.hypot(dx, dy), 0, Math.PI * 2);
        } else {
          // ellipse inscribed in the drag rectangle
          ctx.ellipse(
            from.x + dx / 2,
            from.y + dy / 2,
            Math.abs(dx) / 2,
            Math.abs(dy) / 2,
            0,
            0,
            Math.PI * 2,
          );
        }
      }
      ctx.stroke();
    }

    function saveToStorage() {
      try {
        localStorage.setItem(storageKey, canvas.toDataURL("image/png"));
        localStorage.setItem(bgKey, background);
      } catch (e) {
        /* removal can't throw anything meaningful */
        console.warn("mdbook-draw: could not save to localStorage:", e);
      }
    }

    function clearStorage() {
      try {
        localStorage.removeItem(storageKey);
        localStorage.removeItem(bgKey);
      } catch (e) {}
    }

    function loadFromStorage() {
      try {
        var savedBg = localStorage.getItem(bgKey);
        var saved = localStorage.getItem(storageKey);

        if (!saved) return false;

        // If the background color changed since the save was made,
        // discard the stale save so the new background takes effect.
        if (savedBg !== background) {
          clearStorage();
          return false;
        }

        var img = new Image();
        img.onload = function () {
          ctx.drawImage(img, 0, 0);
        };
        img.src = saved;
        return true;
      } catch (e) {
        console.warn("mdbook-draw: could not load from localStorage:", e);
        return false;
      }
    }

    // --- Init: fill background, then restore saved drawing if any ---

    ctx.fillStyle = background;
    ctx.fillRect(0, 0, canvas.width, canvas.height);
    loadFromStorage();

    // --- Toolbar wiring ---

    toolbar.querySelectorAll("button[data-tool]").forEach(function (btn) {
      btn.addEventListener("click", function () {
        var t = btn.getAttribute("data-tool");
        console.log("tool selected:", t);
        if (t === "clear") {
          ctx.fillStyle = background;
          ctx.fillRect(0, 0, canvas.width, canvas.height);
          clearStorage();
        } else {
          tool = t;
        }
      });
    });

    // Save button
    var saveBtn = toolbar.querySelector("button[data-role='save']");
    if (saveBtn) {
      saveBtn.addEventListener("click", function () {
        saveToStorage();
        var original = saveBtn.textContent;
        saveBtn.textContent = "✅ Saved!";
        setTimeout(function () {
          saveBtn.textContent = original;
        }, 1500);
      });
    }

    // Export PNG button
    var exportBtn = toolbar.querySelector("button[data-role='export-png']");
    if (exportBtn) {
      exportBtn.addEventListener("click", function () {
        var link = document.createElement("a");
        link.download = canvasId + ".png";
        link.href = canvas.toDataURL("image/png");
        link.click();
      });
    }

    var colorInput = toolbar.querySelector("input[data-role='color']");
    if (colorInput) {
      colorInput.addEventListener("input", function () {
        color = colorInput.value;
      });
    }

    var sizeInput = toolbar.querySelector("input[data-role='size']");
    if (sizeInput) {
      sizeInput.addEventListener("input", function () {
        brushSize = parseInt(sizeInput.value, 10);
      });
    }

    // --- Drawing events ---

    function getPos(e) {
      var rect = canvas.getBoundingClientRect();
      var scaleX = canvas.width / rect.width;
      var scaleY = canvas.height / rect.height;
      return {
        x: (e.clientX - rect.left) * scaleX,
        y: (e.clientY - rect.top) * scaleY,
      };
    }

    canvas.addEventListener("mousedown", function (e) {
      drawing = true;
      startPos = getPos(e);
      if (tool === "text") {
        var pos = getPos(e);
        var text = prompt("Enter text:");
        if (text) {
          ctx.fillStyle = color;
          ctx.font = brushSize * 4 + "px sans-serif";
          ctx.fillText(text, pos.x, pos.y);
          saveToStorage();
        }
        drawing = false;
        return;
      }
      if (isShapeTool(tool)) {
        snapshot = ctx.getImageData(0, 0, canvas.width, canvas.height);
      } else {
        ctx.beginPath();
        ctx.moveTo(startPos.x, startPos.y);
      }
    });

    canvas.addEventListener("mousemove", function (e) {
      if (!drawing) return;
      var pos = getPos(e);
      if (isShapeTool(tool)) {
        ctx.putImageData(snapshot, 0, 0); // wipe previous preview
        drawShape(startPos, pos, e.shiftKey);
      } else {
        applyStroke();
        ctx.lineTo(pos.x, pos.y);
        ctx.stroke();
      }
    });

    function finish(e, commit) {
      if (!drawing) return;
      drawing = false;
      if (isShapeTool(tool)) {
        ctx.putImageData(snapshot, 0, 0);
        if (commit) drawShape(startPos, getPos(e), e.shiftKey);
        snapshot = null;
      }
      saveToStorage();
    }

    canvas.addEventListener("mouseup", function (e) {
      finish(e, true);
    });
    canvas.addEventListener("mouseleave", function (e) {
      finish(e, true);
    });
    console.log("mdbook-draw: initialized canvas", canvasId);
  });
})();
