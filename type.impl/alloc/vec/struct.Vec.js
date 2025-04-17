(function() {
    var type_impls = Object.fromEntries([["drgrep",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-IntoIterator-for-%26Vec%3CT,+A%3E\" class=\"impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.86.0/src/alloc/vec/mod.rs.html#3468\">Source</a></span><a href=\"#impl-IntoIterator-for-%26Vec%3CT,+A%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, T, A&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.86.0/core/iter/traits/collect/trait.IntoIterator.html\" title=\"trait core::iter::traits::collect::IntoIterator\">IntoIterator</a> for &amp;'a <a class=\"struct\" href=\"https://doc.rust-lang.org/1.86.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;T, A&gt;<div class=\"where\">where\n    A: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.86.0/core/alloc/trait.Allocator.html\" title=\"trait core::alloc::Allocator\">Allocator</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Item\" class=\"associatedtype trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/1.86.0/src/alloc/vec/mod.rs.html#3469\">Source</a><a href=\"#associatedtype.Item\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/1.86.0/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.86.0/std/primitive.reference.html\">&amp;'a T</a></h4></section></summary><div class='docblock'>The type of the elements being iterated over.</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.IntoIter\" class=\"associatedtype trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/1.86.0/src/alloc/vec/mod.rs.html#3470\">Source</a><a href=\"#associatedtype.IntoIter\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/1.86.0/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter\" class=\"associatedtype\">IntoIter</a> = <a class=\"struct\" href=\"https://doc.rust-lang.org/1.86.0/core/slice/iter/struct.Iter.html\" title=\"struct core::slice::iter::Iter\">Iter</a>&lt;'a, T&gt;</h4></section></summary><div class='docblock'>Which kind of iterator are we turning this into?</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_iter\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/1.86.0/src/alloc/vec/mod.rs.html#3472\">Source</a><a href=\"#method.into_iter\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.86.0/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter\" class=\"fn\">into_iter</a>(self) -&gt; &lt;&amp;'a <a class=\"struct\" href=\"https://doc.rust-lang.org/1.86.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;T, A&gt; as <a class=\"trait\" href=\"https://doc.rust-lang.org/1.86.0/core/iter/traits/collect/trait.IntoIterator.html\" title=\"trait core::iter::traits::collect::IntoIterator\">IntoIterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/1.86.0/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter\" title=\"type core::iter::traits::collect::IntoIterator::IntoIter\">IntoIter</a> <a href=\"#\" class=\"tooltip\" data-notable-ty=\"&lt;&amp;&#39;a Vec&lt;T, A&gt; as IntoIterator&gt;::IntoIter\">ⓘ</a></h4></section></summary><div class='docblock'>Creates an iterator from a value. <a href=\"https://doc.rust-lang.org/1.86.0/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter\">Read more</a></div></details></div></details>","IntoIterator","drgrep::color::printer::TextParts"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[3913]}