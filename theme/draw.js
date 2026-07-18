// draw.js: mdbook-draw canvas initializer
// mdBook injects additional-js scripts at the bottom of <body>, so the DOM
// is fully parsed by the time this runs. No event listener needed.

(function () {
  var STORAGE_PREFIX = "mdbook-draw:";

  var toolbars = document.querySelectorAll(".mdbook-draw-toolbar");

  toolbars.forEach(function (toolbar) {
    var canvasId   = toolbar.getAttribute("data-canvas-id");
    var canvas     = document.getElementById(canvasId);

    if (!canvas) {
      console.warn("mdbook-draw: canvas not found for id:", canvasId);
      return;
    }

    var ctx        = canvas.getContext("2d");
    var drawing    = false;
    var tool       = "pencil";
    var color      = "#000000";
    var brushSize  = 4;
    var background = canvas.getAttribute("data-background") || "#ffffff";
    var storageKey = STORAGE_PREFIX + canvasId;
    var bgKey      = STORAGE_PREFIX + canvasId + ":bg";

    // --- Persistence helpers ---

    function saveToStorage() {
      try {
        localStorage.setItem(storageKey, canvas.toDataURL("image/png"));
        localStorage.setItem(bgKey, background);
      } catch (e) {
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
        var saved   = localStorage.getItem(storageKey);

        if (!saved) return false;

        // If the background color changed since the save was made,
        // discard the stale save so the new background takes effect.
        if (savedBg !== background) {
          clearStorage();
          return false;
        }

        var img = new Image();
        img.onload = function () { ctx.drawImage(img, 0, 0); };
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
        setTimeout(function () { saveBtn.textContent = original; }, 1500);
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
      colorInput.addEventListener("input", function () { color = colorInput.value; });
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
        y: (e.clientY - rect.top)  * scaleY,
      };
    }

    canvas.addEventListener("mousedown", function (e) {
      drawing = true;
      var pos = getPos(e);
      ctx.beginPath();
      ctx.moveTo(pos.x, pos.y);
    });

    canvas.addEventListener("mousemove", function (e) {
      if (!drawing) return;
      var pos = getPos(e);

      if (tool === "eraser") {
        ctx.strokeStyle = background;
        ctx.lineWidth   = brushSize * 3;
      } else {
        ctx.strokeStyle = color;
        ctx.lineWidth   = brushSize;
      }

      ctx.lineCap  = "round";
      ctx.lineJoin = "round";
      ctx.lineTo(pos.x, pos.y);
      ctx.stroke();
    });

    canvas.addEventListener("mouseup", function () {
      drawing = false;
      saveToStorage(); // Auto-save after every stroke
    });

    canvas.addEventListener("mouseleave", function () { drawing = false; });

    console.log("mdbook-draw: initialized canvas", canvasId);
  });
}());
