import("../pkg/index.js").catch(console.error).then(module => {
    const Index = module.Index;
    console.log(Index);
    const i = Index.new(42, 3, 0.5);
    window.i = i;
    i.insert(0, "Hello");
    i.insert(1, "Hello world");
    console.log(i.size());
});