const corpus = [
  "This is the first document.",
  "This document is the second document.",
  "And this is the third document.",
  "Is this the first document?",
  "This not the first nor the second nor the third, but the fourth document"];

import("../pkg/index.js").catch(console.error).then(module => {
    const Index = module.Index;
    console.log(corpus);

    const i = Index.new(42, 3, 0.5);
    window.i = i;

    corpus.forEach((text, idx) => {
        i.insert(idx, text);
    });

    const result = i.query("This is the first");
    console.log(result);

    console.log("size ->", i.size());
});