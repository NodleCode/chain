(function() {var type_impls = {
"runtime_eden":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-BeforeAllRuntimeMigrations-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-BeforeAllRuntimeMigrations-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; BeforeAllRuntimeMigrations for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.before_all_runtime_migrations\" class=\"method trait-impl\"><a href=\"#method.before_all_runtime_migrations\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">before_all_runtime_migrations</a>() -&gt; Weight</h4></section></summary><div class='docblock'>Something that should happen before runtime migrations are executed.</div></details></div></details>","BeforeAllRuntimeMigrations","runtime_eden::MessageQueue"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Callable%3CT%3E-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-Callable%3CT%3E-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; Callable&lt;T&gt; for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><section id=\"associatedtype.RuntimeCall\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.RuntimeCall\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">RuntimeCall</a> = Call&lt;T&gt;</h4></section></div></details>","Callable<T>","runtime_eden::MessageQueue"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-Clone-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.79.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for Pallet&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.79.0/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; Pallet&lt;T&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.79.0/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.79.0/src/core/clone.rs.html#169\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.79.0/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.79.0/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.79.0/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","runtime_eden::MessageQueue"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-Debug-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.79.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for Pallet&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.79.0/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, fmt: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.79.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.79.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.79.0/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.79.0/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.79.0/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","runtime_eden::MessageQueue"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-EnqueueMessage%3C%3C%3CT+as+Config%3E::MessageProcessor+as+ProcessMessage%3E::Origin%3E-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-EnqueueMessage%3C%3C%3CT+as+Config%3E::MessageProcessor+as+ProcessMessage%3E::Origin%3E-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; EnqueueMessage&lt;&lt;&lt;T as Config&gt;::MessageProcessor as ProcessMessage&gt;::Origin&gt; for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.MaxMessageLen\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.MaxMessageLen\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">MaxMessageLen</a> = MaxMessageLen&lt;&lt;&lt;T as Config&gt;::MessageProcessor as ProcessMessage&gt;::Origin, &lt;T as Config&gt;::Size, &lt;T as Config&gt;::HeapSize&gt;</h4></section></summary><div class='docblock'>The maximal length any enqueued message may have.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.enqueue_message\" class=\"method trait-impl\"><a href=\"#method.enqueue_message\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">enqueue_message</a>(\n    message: BoundedSlice&lt;'_, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.79.0/std/primitive.u8.html\">u8</a>, &lt;Pallet&lt;T&gt; as EnqueueMessage&lt;&lt;&lt;T as Config&gt;::MessageProcessor as ProcessMessage&gt;::Origin&gt;&gt;::MaxMessageLen&gt;,\n    origin: &lt;&lt;T as Config&gt;::MessageProcessor as ProcessMessage&gt;::Origin\n)</h4></section></summary><div class='docblock'>Enqueue a single <code>message</code> from a specific <code>origin</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.enqueue_messages\" class=\"method trait-impl\"><a href=\"#method.enqueue_messages\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">enqueue_messages</a>&lt;'a&gt;(\n    messages: impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.79.0/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&lt;Item = BoundedSlice&lt;'a, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.79.0/std/primitive.u8.html\">u8</a>, &lt;Pallet&lt;T&gt; as EnqueueMessage&lt;&lt;&lt;T as Config&gt;::MessageProcessor as ProcessMessage&gt;::Origin&gt;&gt;::MaxMessageLen&gt;&gt;,\n    origin: &lt;&lt;T as Config&gt;::MessageProcessor as ProcessMessage&gt;::Origin\n)</h4></section></summary><div class='docblock'>Enqueue multiple <code>messages</code> from a specific <code>origin</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.sweep_queue\" class=\"method trait-impl\"><a href=\"#method.sweep_queue\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">sweep_queue</a>(\n    origin: &lt;&lt;T as Config&gt;::MessageProcessor as ProcessMessage&gt;::Origin\n)</h4></section></summary><div class='docblock'>Any remaining unprocessed messages should happen only lazily, not proactively.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.footprint\" class=\"method trait-impl\"><a href=\"#method.footprint\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">footprint</a>(\n    origin: &lt;&lt;T as Config&gt;::MessageProcessor as ProcessMessage&gt;::Origin\n) -&gt; QueueFootprint</h4></section></summary><div class='docblock'>Return the state footprint of the given queue.</div></details></div></details>","EnqueueMessage<<<T as Config>::MessageProcessor as ProcessMessage>::Origin>","runtime_eden::MessageQueue"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-GetStorageVersion-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-GetStorageVersion-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; GetStorageVersion for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.CurrentStorageVersion\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.CurrentStorageVersion\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">CurrentStorageVersion</a> = NoStorageVersionSet</h4></section></summary><div class='docblock'>This will be filled out by the <a href=\"crate::pallet\"><code>pallet</code></a> macro. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.current_storage_version\" class=\"method trait-impl\"><a href=\"#method.current_storage_version\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">current_storage_version</a>(\n) -&gt; &lt;Pallet&lt;T&gt; as GetStorageVersion&gt;::CurrentStorageVersion</h4></section></summary><div class='docblock'>Returns the current storage version as supported by the pallet.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.on_chain_storage_version\" class=\"method trait-impl\"><a href=\"#method.on_chain_storage_version\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">on_chain_storage_version</a>() -&gt; StorageVersion</h4></section></summary><div class='docblock'>Returns the on-chain storage version of the pallet as stored in the storage.</div></details></div></details>","GetStorageVersion","runtime_eden::MessageQueue"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Hooks%3C%3C%3C%3CT+as+Config%3E::Block+as+HeaderProvider%3E::HeaderT+as+Header%3E::Number%3E-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-Hooks%3C%3C%3C%3CT+as+Config%3E::Block+as+HeaderProvider%3E::HeaderT+as+Header%3E::Number%3E-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; Hooks&lt;&lt;&lt;&lt;T as Config&gt;::Block as HeaderProvider&gt;::HeaderT as Header&gt;::Number&gt; for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.on_initialize\" class=\"method trait-impl\"><a href=\"#method.on_initialize\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">on_initialize</a>(\n    _n: &lt;&lt;&lt;T as Config&gt;::Block as HeaderProvider&gt;::HeaderT as Header&gt;::Number\n) -&gt; Weight</h4></section></summary><div class='docblock'>Block initialization hook. This is called at the very beginning of block execution. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.on_finalize\" class=\"method trait-impl\"><a href=\"#method.on_finalize\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">on_finalize</a>(_n: BlockNumber)</h4></section></summary><div class='docblock'>Block finalization hook. This is called at the very end of block execution. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.on_idle\" class=\"method trait-impl\"><a href=\"#method.on_idle\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">on_idle</a>(_n: BlockNumber, _remaining_weight: Weight) -&gt; Weight</h4></section></summary><div class='docblock'>Hook to consume a block’s idle time. This will run when the block is being finalized (before\n[<code>Hooks::on_finalize</code>]). <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.on_runtime_upgrade\" class=\"method trait-impl\"><a href=\"#method.on_runtime_upgrade\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">on_runtime_upgrade</a>() -&gt; Weight</h4></section></summary><div class='docblock'>Hook executed when a code change (aka. a “runtime upgrade”) is detected by FRAME. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.offchain_worker\" class=\"method trait-impl\"><a href=\"#method.offchain_worker\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">offchain_worker</a>(_n: BlockNumber)</h4></section></summary><div class='docblock'>Implementing this function on a pallet allows you to perform long-running tasks that are\ndispatched as separate threads, and entirely independent of the main wasm runtime. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.integrity_test\" class=\"method trait-impl\"><a href=\"#method.integrity_test\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">integrity_test</a>()</h4></section></summary><div class='docblock'>Check the integrity of this pallet’s configuration. <a>Read more</a></div></details></div></details>","Hooks<<<<T as Config>::Block as HeaderProvider>::HeaderT as Header>::Number>","runtime_eden::MessageQueue"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-IntegrityTest-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-IntegrityTest-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; IntegrityTest for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.integrity_test\" class=\"method trait-impl\"><a href=\"#method.integrity_test\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">integrity_test</a>()</h4></section></summary><div class='docblock'>See [<code>Hooks::integrity_test</code>].</div></details></div></details>","IntegrityTest","runtime_eden::MessageQueue"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-OffchainWorker%3C%3C%3C%3CT+as+Config%3E::Block+as+HeaderProvider%3E::HeaderT+as+Header%3E::Number%3E-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-OffchainWorker%3C%3C%3C%3CT+as+Config%3E::Block+as+HeaderProvider%3E::HeaderT+as+Header%3E::Number%3E-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; OffchainWorker&lt;&lt;&lt;&lt;T as Config&gt;::Block as HeaderProvider&gt;::HeaderT as Header&gt;::Number&gt; for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.offchain_worker\" class=\"method trait-impl\"><a href=\"#method.offchain_worker\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">offchain_worker</a>(\n    n: &lt;&lt;&lt;T as Config&gt;::Block as HeaderProvider&gt;::HeaderT as Header&gt;::Number\n)</h4></section></summary><div class='docblock'>This function is being called after every block import (when fully synced). <a>Read more</a></div></details></div></details>","OffchainWorker<<<<T as Config>::Block as HeaderProvider>::HeaderT as Header>::Number>","runtime_eden::MessageQueue"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-OnFinalize%3C%3C%3C%3CT+as+Config%3E::Block+as+HeaderProvider%3E::HeaderT+as+Header%3E::Number%3E-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-OnFinalize%3C%3C%3C%3CT+as+Config%3E::Block+as+HeaderProvider%3E::HeaderT+as+Header%3E::Number%3E-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; OnFinalize&lt;&lt;&lt;&lt;T as Config&gt;::Block as HeaderProvider&gt;::HeaderT as Header&gt;::Number&gt; for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.on_finalize\" class=\"method trait-impl\"><a href=\"#method.on_finalize\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">on_finalize</a>(\n    n: &lt;&lt;&lt;T as Config&gt;::Block as HeaderProvider&gt;::HeaderT as Header&gt;::Number\n)</h4></section></summary><div class='docblock'>See [<code>Hooks::on_finalize</code>].</div></details></div></details>","OnFinalize<<<<T as Config>::Block as HeaderProvider>::HeaderT as Header>::Number>","runtime_eden::MessageQueue"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-OnGenesis-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-OnGenesis-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; OnGenesis for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.on_genesis\" class=\"method trait-impl\"><a href=\"#method.on_genesis\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">on_genesis</a>()</h4></section></summary><div class='docblock'>Something that should happen at genesis.</div></details></div></details>","OnGenesis","runtime_eden::MessageQueue"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-OnIdle%3C%3C%3C%3CT+as+Config%3E::Block+as+HeaderProvider%3E::HeaderT+as+Header%3E::Number%3E-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-OnIdle%3C%3C%3C%3CT+as+Config%3E::Block+as+HeaderProvider%3E::HeaderT+as+Header%3E::Number%3E-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; OnIdle&lt;&lt;&lt;&lt;T as Config&gt;::Block as HeaderProvider&gt;::HeaderT as Header&gt;::Number&gt; for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.on_idle\" class=\"method trait-impl\"><a href=\"#method.on_idle\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">on_idle</a>(\n    n: &lt;&lt;&lt;T as Config&gt;::Block as HeaderProvider&gt;::HeaderT as Header&gt;::Number,\n    remaining_weight: Weight\n) -&gt; Weight</h4></section></summary><div class='docblock'>See [<code>Hooks::on_idle</code>].</div></details></div></details>","OnIdle<<<<T as Config>::Block as HeaderProvider>::HeaderT as Header>::Number>","runtime_eden::MessageQueue"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-OnInitialize%3C%3C%3C%3CT+as+Config%3E::Block+as+HeaderProvider%3E::HeaderT+as+Header%3E::Number%3E-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-OnInitialize%3C%3C%3C%3CT+as+Config%3E::Block+as+HeaderProvider%3E::HeaderT+as+Header%3E::Number%3E-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; OnInitialize&lt;&lt;&lt;&lt;T as Config&gt;::Block as HeaderProvider&gt;::HeaderT as Header&gt;::Number&gt; for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.on_initialize\" class=\"method trait-impl\"><a href=\"#method.on_initialize\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">on_initialize</a>(\n    n: &lt;&lt;&lt;T as Config&gt;::Block as HeaderProvider&gt;::HeaderT as Header&gt;::Number\n) -&gt; Weight</h4></section></summary><div class='docblock'>See [<code>Hooks::on_initialize</code>].</div></details></div></details>","OnInitialize<<<<T as Config>::Block as HeaderProvider>::HeaderT as Header>::Number>","runtime_eden::MessageQueue"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-OnRuntimeUpgrade-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-OnRuntimeUpgrade-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; OnRuntimeUpgrade for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.on_runtime_upgrade\" class=\"method trait-impl\"><a href=\"#method.on_runtime_upgrade\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">on_runtime_upgrade</a>() -&gt; Weight</h4></section></summary><div class='docblock'>See [<code>Hooks::on_runtime_upgrade</code>].</div></details></div></details>","OnRuntimeUpgrade","runtime_eden::MessageQueue"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.do_execute_overweight\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">do_execute_overweight</a>(\n    origin: &lt;&lt;T as Config&gt;::MessageProcessor as ProcessMessage&gt;::Origin,\n    page_index: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.79.0/std/primitive.u32.html\">u32</a>,\n    index: &lt;T as Config&gt;::Size,\n    weight_limit: Weight\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.79.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;Weight, Error&lt;T&gt;&gt;</h4></section></summary><div class=\"docblock\"><p>Try to execute a single message that was marked as overweight.</p>\n<p>The <code>weight_limit</code> is the weight that can be consumed to execute the message. The base\nweight of the function it self must be measured by the caller.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.do_try_state\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">do_try_state</a>() -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.79.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.79.0/std/primitive.unit.html\">()</a>, DispatchError&gt;</h4></section></summary><div class=\"docblock\"><p>Ensure the correctness of state of this pallet.</p>\n<h5 id=\"assumptions-\"><a class=\"doc-anchor\" href=\"#assumptions-\">§</a>Assumptions-</h5>\n<p>If <code>serviceHead</code> points to a ready Queue, then BookState of that Queue has:</p>\n<ul>\n<li><code>message_count</code> &gt; 0</li>\n<li><code>size</code> &gt; 0</li>\n<li><code>end</code> &gt; <code>begin</code></li>\n<li>Some(ready_neighbours)</li>\n<li>If <code>ready_neighbours.next</code> == self.origin, then <code>ready_neighbours.prev</code> == self.origin\n(only queue in ring)</li>\n</ul>\n<p>For Pages(begin to end-1) in BookState:</p>\n<ul>\n<li><code>remaining</code> &gt; 0</li>\n<li><code>remaining_size</code> &gt; 0</li>\n<li><code>first</code> &lt;= <code>last</code></li>\n<li>Every page can be decoded into peek_* functions</li>\n</ul>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.debug_info\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">debug_info</a>() -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.79.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a></h4></section></summary><div class=\"docblock\"><p>Print the pages in each queue and the messages in each page.</p>\n<p>Processed messages are prefixed with a <code>*</code> and the current <code>begin</code>ning page with a <code>&gt;</code>.</p>\n<h5 id=\"example-output\"><a class=\"doc-anchor\" href=\"#example-output\">§</a>Example output</h5><div class=\"example-wrap\"><pre class=\"language-text\"><code>queue Here:\n  page 0: []\n&gt; page 1: []\n  page 2: [&quot;\\0weight=4&quot;, &quot;\\0c&quot;, ]\n  page 3: [&quot;\\0bigbig 1&quot;, ]\n  page 4: [&quot;\\0bigbig 2&quot;, ]\n  page 5: [&quot;\\0bigbig 3&quot;, ]\n</code></pre></div></div></details></div></details>",0,"runtime_eden::MessageQueue"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.reap_page\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">reap_page</a>(\n    origin: &lt;T as Config&gt;::RuntimeOrigin,\n    message_origin: &lt;&lt;T as Config&gt;::MessageProcessor as ProcessMessage&gt;::Origin,\n    page_index: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.79.0/std/primitive.u32.html\">u32</a>\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.79.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.79.0/std/primitive.unit.html\">()</a>, DispatchError&gt;</h4></section></summary><div class=\"docblock\"><p>Remove a page which has no more messages remaining to be processed or is stale.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.execute_overweight\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">execute_overweight</a>(\n    origin: &lt;T as Config&gt;::RuntimeOrigin,\n    message_origin: &lt;&lt;T as Config&gt;::MessageProcessor as ProcessMessage&gt;::Origin,\n    page: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.79.0/std/primitive.u32.html\">u32</a>,\n    index: &lt;T as Config&gt;::Size,\n    weight_limit: Weight\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.79.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;PostDispatchInfo, DispatchErrorWithPostInfo&lt;PostDispatchInfo&gt;&gt;</h4></section></summary><div class=\"docblock\"><p>Execute an overweight message.</p>\n<p>Temporary processing errors will be propagated whereas permanent errors are treated\nas success condition.</p>\n<ul>\n<li><code>origin</code>: Must be <code>Signed</code>.</li>\n<li><code>message_origin</code>: The origin from which the message to be executed arrived.</li>\n<li><code>page</code>: The page in the queue in which the message to be executed is sitting.</li>\n<li><code>index</code>: The index into the queue of the message to be executed.</li>\n<li><code>weight_limit</code>: The maximum amount of weight allowed to be consumed in the execution\nof the message.</li>\n</ul>\n<p>Benchmark complexity considerations: O(index + weight_limit).</p>\n</div></details></div></details>",0,"runtime_eden::MessageQueue"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PalletInfoAccess-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-PalletInfoAccess-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; PalletInfoAccess for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.index\" class=\"method trait-impl\"><a href=\"#method.index\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">index</a>() -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.79.0/std/primitive.usize.html\">usize</a></h4></section></summary><div class='docblock'>Index of the pallet as configured in the runtime.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.name\" class=\"method trait-impl\"><a href=\"#method.name\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">name</a>() -&gt; &amp;'static <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.79.0/std/primitive.str.html\">str</a></h4></section></summary><div class='docblock'>Name of the pallet as configured in the runtime.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.name_hash\" class=\"method trait-impl\"><a href=\"#method.name_hash\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">name_hash</a>() -&gt; [<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.79.0/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.79.0/std/primitive.array.html\">16</a>]</h4></section></summary><div class='docblock'>Two128 hash of name.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.module_name\" class=\"method trait-impl\"><a href=\"#method.module_name\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">module_name</a>() -&gt; &amp;'static <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.79.0/std/primitive.str.html\">str</a></h4></section></summary><div class='docblock'>Name of the Rust module containing the pallet.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.crate_version\" class=\"method trait-impl\"><a href=\"#method.crate_version\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">crate_version</a>() -&gt; CrateVersion</h4></section></summary><div class='docblock'>Version of the crate containing the pallet.</div></details></div></details>","PalletInfoAccess","runtime_eden::MessageQueue"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PalletsInfoAccess-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-PalletsInfoAccess-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; PalletsInfoAccess for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.count\" class=\"method trait-impl\"><a href=\"#method.count\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">count</a>() -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.79.0/std/primitive.usize.html\">usize</a></h4></section></summary><div class='docblock'>The number of pallets’ information that this type represents. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.infos\" class=\"method trait-impl\"><a href=\"#method.infos\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">infos</a>() -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.79.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;PalletInfoData&gt;</h4></section></summary><div class='docblock'>All of the pallets’ information that this type represents.</div></details></div></details>","PalletsInfoAccess","runtime_eden::MessageQueue"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-PartialEq-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.79.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> for Pallet&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.79.0/core/cmp/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: &amp;Pallet&lt;T&gt;) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.79.0/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>self</code> and <code>other</code> values to be equal, and is used\nby <code>==</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.79.0/src/core/cmp.rs.html#263\">source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.79.0/core/cmp/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.79.0/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.79.0/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>!=</code>. The default implementation is almost always\nsufficient, and should not be overridden without very good reason.</div></details></div></details>","PartialEq","runtime_eden::MessageQueue"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-ServiceQueues-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-ServiceQueues-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; ServiceQueues for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.execute_overweight\" class=\"method trait-impl\"><a href=\"#method.execute_overweight\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">execute_overweight</a>(\n    weight_limit: Weight,\n    _: &lt;Pallet&lt;T&gt; as ServiceQueues&gt;::OverweightMessageAddress\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.79.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;Weight, ExecuteOverweightError&gt;</h4></section></summary><div class=\"docblock\"><p>Execute a single overweight message.</p>\n<p>The weight limit must be enough for <code>execute_overweight</code> and the message execution itself.</p>\n</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.OverweightMessageAddress\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.OverweightMessageAddress\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">OverweightMessageAddress</a> = (&lt;&lt;T as Config&gt;::MessageProcessor as ProcessMessage&gt;::Origin, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.79.0/std/primitive.u32.html\">u32</a>, &lt;T as Config&gt;::Size)</h4></section></summary><div class='docblock'>Addresses a specific overweight message.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.service_queues\" class=\"method trait-impl\"><a href=\"#method.service_queues\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">service_queues</a>(weight_limit: Weight) -&gt; Weight</h4></section></summary><div class='docblock'>Service all message queues in some fair manner. <a>Read more</a></div></details></div></details>","ServiceQueues","runtime_eden::MessageQueue"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-StorageInfoTrait-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-StorageInfoTrait-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; StorageInfoTrait for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><section id=\"method.storage_info\" class=\"method trait-impl\"><a href=\"#method.storage_info\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">storage_info</a>() -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.79.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;StorageInfo&gt;</h4></section></div></details>","StorageInfoTrait","runtime_eden::MessageQueue"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-WhitelistedStorageKeys-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-WhitelistedStorageKeys-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; WhitelistedStorageKeys for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.whitelisted_storage_keys\" class=\"method trait-impl\"><a href=\"#method.whitelisted_storage_keys\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">whitelisted_storage_keys</a>() -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.79.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;TrackedStorageKey&gt;</h4></section></summary><div class='docblock'>Returns a <a href=\"https://doc.rust-lang.org/1.79.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\"><code>Vec&lt;TrackedStorageKey&gt;</code></a> indicating the storage keys that\nshould be whitelisted during benchmarking. This means that those keys\nwill be excluded from the benchmarking performance calculation.</div></details></div></details>","WhitelistedStorageKeys","runtime_eden::MessageQueue"],["<section id=\"impl-Eq-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-Eq-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.79.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> for Pallet&lt;T&gt;</h3></section>","Eq","runtime_eden::MessageQueue"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()