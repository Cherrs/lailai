let origOpen = XMLHttpRequest.prototype.open;

window.addEventListener('message', function (e) {
    if (e.data != undefined) {
        let data = JSON.parse(e.data);
        if (data.message.type == 34) {
            window.ipc.postMessage(data.message.ticket);
        }
    }

}, false);

XMLHttpRequest.prototype.open = function () {
    this.addEventListener('load', async function () {
        if (this.responseURL == 'https://t.captcha.qq.com/cap_union_new_verify') {
            let j = JSON.parse(this.responseText);
            if (j.errorCode == '0') {
                top.postMessage(j.ticket);
            }
        }
    });
    origOpen.apply(this, arguments);
}