// Animated background
function createFloatingPixels() {
    const bgAnimation = document.getElementById('bgAnimation');
    const pixelCount = 50;

    for (let i = 0; i < pixelCount; i++) {
        const pixel = document.createElement('div');
        pixel.className = 'floating-pixel';
        pixel.style.left = Math.random() * 100 + '%';
        pixel.style.animationDelay = Math.random() * 20 + 's';
        pixel.style.animationDuration = (15 + Math.random() * 10) + 's';
        bgAnimation.appendChild(pixel);
    }
}

// Tab functionality
function showTab(tabName) {
    // Hide all tabs
    document.querySelectorAll('.tab-content').forEach(tab => {
        tab.style.display = 'none';
    });

    // Remove active class from all buttons
    document.querySelectorAll('.tab-btn').forEach(btn => {
        btn.classList.remove('active');
    });

    // Show selected tab
    document.getElementById(tabName + '-tab').style.display = 'block';

    // Add active class to clicked button
    event.target.classList.add('active');
}

function copyCode(button) {
    const codeBlock = button.parentElement.querySelector('pre');
    const text = codeBlock.textContent;
    const originalText = button.textContent;

    if (navigator.clipboard && navigator.clipboard.writeText) {
        navigator.clipboard.writeText(text).then(() => {
            button.textContent = 'Copied!';
            button.style.background = '#00ff88';
            button.style.color = '#000000';
            setTimeout(() => {
                button.textContent = originalText;
                button.style.background = 'rgba(0, 255, 136, 0.1)';
                button.style.color = '#00ff88';
            }, 2000);
        }).catch(() => {
            showCopyError(button, originalText);
        });
    } else {
        // Fallback for older browsers
        const textarea = document.createElement('textarea');
        textarea.value = text;
        document.body.appendChild(textarea);
        textarea.select();
        try {
            document.execCommand('copy');
            button.textContent = 'Copied!';
            button.style.background = '#00ff88';
            button.style.color = '#000000';
            setTimeout(() => {
                button.textContent = originalText;
                button.style.background = 'rgba(0, 255, 136, 0.1)';
                button.style.color = '#00ff88';
            }, 2000);
        } catch (err) {
            showCopyError(button, originalText);
        }
        document.body.removeChild(textarea);
    }
}

function showCopyError(button, originalText) {
    button.textContent = 'Failed!';
    button.style.background = '#ff4444';
    button.style.color = '#ffffff';
    setTimeout(() => {
        button.textContent = originalText;
        button.style.background = 'rgba(0, 255, 136, 0.1)';
        button.style.color = '#00ff88';
    }, 2000);
}
// Smooth scrolling for anchor links
document.querySelectorAll('a[href^="#"]').forEach(anchor => {
    anchor.addEventListener('click', function (e) {
        e.preventDefault();
        const target = document.querySelector(this.getAttribute('href'));
        if (target) {
            target.scrollIntoView({
                behavior: 'smooth',
                block: 'start'
            });
        }
    });
});

// Initialize
document.addEventListener('DOMContentLoaded', function () {
    createFloatingPixels();
});
