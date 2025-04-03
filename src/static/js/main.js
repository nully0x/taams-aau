document.addEventListener('DOMContentLoaded', function() {
    const hamburger = document.querySelector('.hamburger-menu');
    const navItems = document.querySelector('.nav-items');
    
    hamburger.addEventListener('click', function() {
        navItems.classList.toggle('active');
    });
});