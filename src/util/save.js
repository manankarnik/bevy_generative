export function save(data) {
	let a = document.createElement("a");
	let blob = new Blob([data], { type: "moel/gltf-binary" });
	console.log(blob.arrayBuffer());
	a.href = window.URL.createObjectURL(blob);
	a.download = "model.glb";
	document.getElementsByTagName("BODY")[0].appendChild(a);
	a.click();
}

