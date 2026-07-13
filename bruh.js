(function() {
    var style = document.createElement('style');
    style.innerHTML = 'img[src$=".bruh"] { visibility: hidden !important; }';
    document.head.appendChild(style);
    var cache = {};

    function processImg(img) {
        var url = img.getAttribute('src');
        if (!url || img.getAttribute('data-bruh-loaded')) return;
        img.setAttribute('data-bruh-loaded', 'true');

        if (cache[url]) {
            swap(img, cache[url]);
            return;
        }

        fetch(url).then(function(res) { return res.arrayBuffer(); }).then(function(buf) {
            var v = new DataView(buf);
            var iw = v.getUint32(0, true), ih = v.getUint32(4, true);
            
            var canvas = document.createElement('canvas');
            canvas.width = iw; canvas.height = ih;
            var ctx = canvas.getContext('2d');
            var imgData = ctx.createImageData(iw, ih);
            var d32 = new Uint32Array(imgData.data.buffer);

            var text = new TextDecoder().decode(buf.slice(8)).replace(/\s+/g, '');
            for (var i = 0; i < iw * ih; i++) {
                var idx = i * 6;
                var r = (h2i(text[idx]) << 4) | h2i(text[idx+1]);
                var g = (h2i(text[idx+2]) << 4) | h2i(text[idx+3]);
                var b = (h2i(text[idx+4]) << 4) | h2i(text[idx+5]);
                d32[i] = (255 << 24) | (b << 16) | (g << 8) | r;
            }

            ctx.putImageData(imgData, 0, 0);
            
            cache[url] = canvas.toDataURL();
            swap(img, cache[url]);
        });
    }

    function h2i(h) {
        var c = h.toLowerCase().charCodeAt(0);
        return c <= 57 ? c - 48 : c - 87;
    }

    function swap(img, dataUrl) {
        var dw = img.getAttribute('width'), dh = img.getAttribute('height');
        var newImg = new Image();
        newImg.src = dataUrl;
        newImg.style.cssText = (img.style.cssText || "") + "; visibility: visible !important;";
        if (dw) newImg.style.width = (dw.indexOf('%') !== -1 ? dw : dw + 'px');
        if (dh) newImg.style.height = (dh.indexOf('%') !== -1 ? dh : dh + 'px');
        
        for (var i = 0; i < img.attributes.length; i++) {
            var a = img.attributes[i];
            if (['src', 'width', 'height', 'style'].indexOf(a.name) === -1) newImg.setAttribute(a.name, a.value);
        }
        img.parentNode.replaceChild(newImg, img);
    }

    function run() {
        var imgs = document.getElementsByTagName('img');
        for (var i = 0; i < imgs.length; i++) {
            if ((imgs[i].getAttribute('src') || "").indexOf('.bruh') !== -1) processImg(imgs[i]);
        }
    }

    if (document.readyState === 'complete') run(); else window.addEventListener('load', run);
})();