function handleChange(field, data) {
    let body = field + ':' + data;
    return fetch("", { method: "PUT", body });
}