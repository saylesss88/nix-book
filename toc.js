// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="Getting_Started_with_Nix_1.html"><strong aria-hidden="true">1.</strong> Getting Started with the Nix Ecosystem</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="nix/nix_language.html"><strong aria-hidden="true">1.1.</strong> Nix Language</a></li><li class="chapter-item expanded "><a href="nix/nix_package_manager.html"><strong aria-hidden="true">1.2.</strong> Nix Package Manager</a></li><li class="chapter-item expanded "><a href="nix/minimal_install.html"><strong aria-hidden="true">1.3.</strong> Minimal Btrfs-Subvol Install with Disko and Flakes</a></li><li class="chapter-item expanded "><a href="nix/impermanence.html"><strong aria-hidden="true">1.4.</strong> Btrfs Impermanence</a></li></ol></li><li class="chapter-item expanded "><a href="Understanding_Nix_Functions_2.html"><strong aria-hidden="true">2.</strong> Understanding Nix Functions</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="functions/practical_functions_2.1.html"><strong aria-hidden="true">2.1.</strong> Practical Nix Functions</a></li><li class="chapter-item expanded "><a href="functions/functions_and_modules_2.2.html"><strong aria-hidden="true">2.2.</strong> Functions and NixOS Modules</a></li></ol></li><li class="chapter-item expanded "><a href="NixOS_Modules_Explained_3.html"><strong aria-hidden="true">3.</strong> NixOS Modules Explained</a></li><li class="chapter-item expanded "><a href="Nix_Flakes_Explained_4.html"><strong aria-hidden="true">4.</strong> Nix Flakes Explained</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="flakes/flake_inputs_4.1.html"><strong aria-hidden="true">4.1.</strong> Nix Flake Inputs</a></li><li class="chapter-item expanded "><a href="flakes/flake_outputs_4.2.html"><strong aria-hidden="true">4.2.</strong> Nix Flake Outputs</a></li><li class="chapter-item expanded "><a href="flakes/flake_examples_4.3.html"><strong aria-hidden="true">4.3.</strong> Nix Flake Examples</a></li><li class="chapter-item expanded "><a href="flakes/helix_flake_4.4.html"><strong aria-hidden="true">4.4.</strong> Hack the Helix Flake</a></li><li class="chapter-item expanded "><a href="flakes/overlays_4.5.html"><strong aria-hidden="true">4.5.</strong> Extending Flakes with Custom Packages using Overlays</a></li></ol></li><li class="chapter-item expanded "><a href="Understanding_Top-Level_Attributes_5.html"><strong aria-hidden="true">5.</strong> Understanding Top-Level Attributes</a></li><li class="chapter-item expanded "><a href="Package_Definitions_Explained_6.html"><strong aria-hidden="true">6.</strong> Package Definitions Explained</a></li><li class="chapter-item expanded "><a href="Intro_to_Nix_Derivations_7.html"><strong aria-hidden="true">7.</strong> Intro to Nix Derivations</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="drv/builders_and_autotools.html"><strong aria-hidden="true">7.1.</strong> Builders and Autotools</a></li></ol></li><li class="chapter-item expanded "><a href="Comparing_Flakes_and_Traditional_Nix_8.html"><strong aria-hidden="true">8.</strong> Comparing Flakes and Traditional Nix</a></li><li class="chapter-item expanded "><a href="Debugging_and_Tracing_NixOS_Modules_9.html"><strong aria-hidden="true">9.</strong> Debugging and Tracing NixOS Modules</a></li><li class="chapter-item expanded "><a href="Working_with_Nixpkgs_Locally_10.html"><strong aria-hidden="true">10.</strong> Working with Nixpkgs Locally</a></li><li class="chapter-item expanded "><a href="Nix_Pull_Requests_11.html"><strong aria-hidden="true">11.</strong> Nix Pull Requests</a></li><li class="chapter-item expanded "><a href="intro_to_nushell_on_NixOS.html"><strong aria-hidden="true">12.</strong> Intro to Nushell on NixOS</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
