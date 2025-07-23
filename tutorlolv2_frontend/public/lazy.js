const elementObserver = new IntersectionObserver((entries, observer) => {
    for (let i = 0; i < entries.length; i++) {
        const entry = entries[i];
        if (entry.isIntersecting) {
            const el = entry.target;
            const sprite = el.getAttribute("data-sprite");
            if (sprite) {
                el.style.backgroundImage = sprite;
                el.removeAttribute("data-sprite");
            }
            observer.unobserve(el);
        }
    }
}, {
    root: null,
    rootMargin: "0px",
    threshold: 0
});

const mutationObserver = new MutationObserver((mutations) => {
    for (let i = 0; i < mutations.length; i++) {
        const mutation = mutations[i];
        for (let j = 0; j < mutation.addedNodes.length; j++) {
            const node = mutation.addedNodes[j];
            if (node.nodeType === Node.ELEMENT_NODE) {
                if (node.hasAttribute("data-sprite")) {
                    elementObserver.observe(node);
                }
                const nested = node.querySelectorAll("[data-sprite]");
                for (let k = 0; k < nested.length; k++) {
                    elementObserver.observe(nested[k]);
                }
            }
        }
    }
});

document.addEventListener("DOMContentLoaded", () => {
    const deferred = document.querySelectorAll("[data-sprite]");
    for (let i = 0; i < deferred.length; i++) {
        elementObserver.observe(deferred[i]);
    }
    mutationObserver.observe(document.body, { childList: true, subtree: true });
});
