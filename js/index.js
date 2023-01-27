import("../pkg/index.js").catch(console.error).then(m => {
    const Index = m.Index;
    const i = Index.new(42, 3);
    window.i = i;
    i.insert(0, "Hello");
    i.insert(1, "Hello world");
    console.log(i.size());
});