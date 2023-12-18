export function save(data, filename, type) {
	let a = document.createElement("a");
	let blob = new Blob([data], { type: type });
	a.href = window.URL.createObjectURL(blob);
	a.download = filename;
	document.getElementsByTagName("BODY")[0].appendChild(a);
	a.click();
}

