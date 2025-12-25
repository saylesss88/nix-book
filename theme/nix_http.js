document.addEventListener("DOMContentLoaded", () => {
  const endpoint = window.NIX_REPL_ENDPOINT || "http://127.0.0.1:8080/";
  const token = window.NIX_REPL_TOKEN || ""; // 1. Read the token

  document.querySelectorAll(".nix-repl-block").forEach((block) => {
    const btn = block.querySelector(".nix-repl-run");
    const codeEl = block.querySelector("code");
    const out = block.querySelector(".nix-repl-output");
    const status = block.querySelector(".nix-repl-status");

    if (!btn || !codeEl || !out) return;

    btn.addEventListener("click", async () => {
      const code = codeEl.textContent;
      block.classList.add("running");
      status.textContent = "Running…";
      out.textContent = "";

      try {
        const res = await fetch(endpoint, {
          method: "POST",
          headers: { 
            "Content-Type": "application/json",
            "X-Nix-Repl-Token": token // 2. Send the token
          },
          body: JSON.stringify({ code }),
        });

        // console.log("nix-repl response status", res.status);
        const raw = await res.text();
        // console.log("nix-repl raw response", raw);

        let data;
        try {
          data = JSON.parse(raw);
        } catch (e) {
          block.classList.add("error");
          out.textContent = "Non-JSON response: " + raw;
          status.textContent = "Error";
          return;
        }

        if (data.error) {
          block.classList.add("error");
          out.textContent = data.error;
          status.textContent = "Error";
        } else {
          block.classList.remove("error");
          out.textContent = data.stdout || "";
          status.textContent = "Done";
        }
      } catch (e) {
        block.classList.add("error");
        out.textContent = String(e);
        status.textContent = "Network error";
      } finally {
        block.classList.remove("running");
      }
    });
  });
});
