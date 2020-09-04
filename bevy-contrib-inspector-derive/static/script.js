const throttle = (func, limit) => {
    if (!limit) return func;

    let lastFunc, lastRan;
    return function () {
        const context = this, args = arguments;
        if (!lastRan) {
            func.apply(context, args);
            lastRan = Date.now();
        } else {
            clearTimeout(lastFunc);
            lastFunc = setTimeout(function () {
                if ((Date.now() - lastRan) >= limit) {
                    func.apply(context, args);
                    lastRan = Date.now();
                }
            }, limit - (Date.now() - lastRan));
        }
    }
}

const handleChange = throttle((field, data) => {
    let body = field + ':' + data;
    return fetch("", { method: "PUT", body }).catch(e => {
        console.error(e);
        alert(e);
    })
}, handleChangeThrottle); // set in inspectable.rs