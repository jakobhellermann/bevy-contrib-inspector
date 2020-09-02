function handleChange(field, data) {
    let body = field + ':' + data;
    return fetch("", { method: "PUT", body }).catch(e => {
        console.error(e);
        alert(e);
    })
}