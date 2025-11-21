// Back-to-top button
const btn = document.createElement('a');
btn.href = '#';
btn.id = 'back-to-top';
btn.textContent = 'Up';
btn.title = 'Back to top';
document.body.appendChild(btn);

// Smooth scroll
btn.addEventListener('click', e => {
    e.preventDefault();
    window.scrollTo({ top: 0, behavior: 'smooth' });
});

// Show/hide on scroll
window.addEventListener('scroll', () => {
    btn.classList.toggle('show', window.scrollY > 300);
});
