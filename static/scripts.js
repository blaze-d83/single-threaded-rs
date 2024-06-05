document.addEventListener('DOMContentLoaded', function () {
    const boxes = document.querySelectorAll('.box h2');

    boxes.forEach(box => {
        box.addEventListener('click', function () {
            const content = this.nextElementSibling;
            if (content.classList.contains('hidden')) {
                content.classList.remove('hidden');
            } else {
                content.classList.add('hidden');
            }
        });
    });
});

