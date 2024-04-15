(function() {var type_impls = {
"runtime_eden":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.next_fee_multiplier\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">next_fee_multiplier</a>() -&gt; FixedU128</h4></section></summary><div class=\"docblock\"><p>An auto-generated getter for [<code>NextFeeMultiplier</code>].</p>\n</div></details></div></details>",0,"runtime_eden::TransactionPayment"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.query_info\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">query_info</a>&lt;Extrinsic&gt;(\n    unchecked_extrinsic: Extrinsic,\n    len: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u32.html\">u32</a>\n) -&gt; RuntimeDispatchInfo&lt;&lt;&lt;T as Config&gt;::OnChargeTransaction as OnChargeTransaction&lt;T&gt;&gt;::Balance&gt;<div class=\"where\">where\n    Extrinsic: Extrinsic + GetDispatchInfo,\n    &lt;T as Config&gt;::RuntimeCall: Dispatchable&lt;Info = DispatchInfo&gt;,</div></h4></section></summary><div class=\"docblock\"><p>Query the data that we know about the fee of a given <code>call</code>.</p>\n<p>This pallet is not and cannot be aware of the internals of a signed extension, for example\na tip. It only interprets the extrinsic as some encoded value and accounts for its weight\nand length, the runtime’s extrinsic base weight, and the current fee multiplier.</p>\n<p>All dispatchables must be annotated with weight and will have some fee info. This function\nalways returns.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.query_fee_details\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">query_fee_details</a>&lt;Extrinsic&gt;(\n    unchecked_extrinsic: Extrinsic,\n    len: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u32.html\">u32</a>\n) -&gt; FeeDetails&lt;&lt;&lt;T as Config&gt;::OnChargeTransaction as OnChargeTransaction&lt;T&gt;&gt;::Balance&gt;<div class=\"where\">where\n    Extrinsic: Extrinsic + GetDispatchInfo,\n    &lt;T as Config&gt;::RuntimeCall: Dispatchable&lt;Info = DispatchInfo&gt;,</div></h4></section></summary><div class=\"docblock\"><p>Query the detailed fee of a given <code>call</code>.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.query_call_info\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">query_call_info</a>(\n    call: &lt;T as Config&gt;::RuntimeCall,\n    len: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u32.html\">u32</a>\n) -&gt; RuntimeDispatchInfo&lt;&lt;&lt;T as Config&gt;::OnChargeTransaction as OnChargeTransaction&lt;T&gt;&gt;::Balance&gt;<div class=\"where\">where\n    &lt;T as Config&gt;::RuntimeCall: Dispatchable&lt;Info = DispatchInfo&gt; + GetDispatchInfo,</div></h4></section></summary><div class=\"docblock\"><p>Query information of a dispatch class, weight, and fee of a given encoded <code>Call</code>.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.query_call_fee_details\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">query_call_fee_details</a>(\n    call: &lt;T as Config&gt;::RuntimeCall,\n    len: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u32.html\">u32</a>\n) -&gt; FeeDetails&lt;&lt;&lt;T as Config&gt;::OnChargeTransaction as OnChargeTransaction&lt;T&gt;&gt;::Balance&gt;<div class=\"where\">where\n    &lt;T as Config&gt;::RuntimeCall: Dispatchable&lt;Info = DispatchInfo&gt; + GetDispatchInfo,</div></h4></section></summary><div class=\"docblock\"><p>Query fee details of a given encoded <code>Call</code>.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.compute_fee\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">compute_fee</a>(\n    len: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u32.html\">u32</a>,\n    info: &amp;&lt;&lt;T as Config&gt;::RuntimeCall as Dispatchable&gt;::Info,\n    tip: &lt;&lt;T as Config&gt;::OnChargeTransaction as OnChargeTransaction&lt;T&gt;&gt;::Balance\n) -&gt; &lt;&lt;T as Config&gt;::OnChargeTransaction as OnChargeTransaction&lt;T&gt;&gt;::Balance<div class=\"where\">where\n    &lt;T as Config&gt;::RuntimeCall: Dispatchable&lt;Info = DispatchInfo&gt;,</div></h4></section></summary><div class=\"docblock\"><p>Compute the final fee value for a particular transaction.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.compute_fee_details\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">compute_fee_details</a>(\n    len: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u32.html\">u32</a>,\n    info: &amp;&lt;&lt;T as Config&gt;::RuntimeCall as Dispatchable&gt;::Info,\n    tip: &lt;&lt;T as Config&gt;::OnChargeTransaction as OnChargeTransaction&lt;T&gt;&gt;::Balance\n) -&gt; FeeDetails&lt;&lt;&lt;T as Config&gt;::OnChargeTransaction as OnChargeTransaction&lt;T&gt;&gt;::Balance&gt;<div class=\"where\">where\n    &lt;T as Config&gt;::RuntimeCall: Dispatchable&lt;Info = DispatchInfo&gt;,</div></h4></section></summary><div class=\"docblock\"><p>Compute the fee details for a particular transaction.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.compute_actual_fee\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">compute_actual_fee</a>(\n    len: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u32.html\">u32</a>,\n    info: &amp;&lt;&lt;T as Config&gt;::RuntimeCall as Dispatchable&gt;::Info,\n    post_info: &amp;&lt;&lt;T as Config&gt;::RuntimeCall as Dispatchable&gt;::PostInfo,\n    tip: &lt;&lt;T as Config&gt;::OnChargeTransaction as OnChargeTransaction&lt;T&gt;&gt;::Balance\n) -&gt; &lt;&lt;T as Config&gt;::OnChargeTransaction as OnChargeTransaction&lt;T&gt;&gt;::Balance<div class=\"where\">where\n    &lt;T as Config&gt;::RuntimeCall: Dispatchable&lt;Info = DispatchInfo, PostInfo = PostDispatchInfo&gt;,</div></h4></section></summary><div class=\"docblock\"><p>Compute the actual post dispatch fee for a particular transaction.</p>\n<p>Identical to <code>compute_fee</code> with the only difference that the post dispatch corrected\nweight is used for the weight fee calculation.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.compute_actual_fee_details\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">compute_actual_fee_details</a>(\n    len: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u32.html\">u32</a>,\n    info: &amp;&lt;&lt;T as Config&gt;::RuntimeCall as Dispatchable&gt;::Info,\n    post_info: &amp;&lt;&lt;T as Config&gt;::RuntimeCall as Dispatchable&gt;::PostInfo,\n    tip: &lt;&lt;T as Config&gt;::OnChargeTransaction as OnChargeTransaction&lt;T&gt;&gt;::Balance\n) -&gt; FeeDetails&lt;&lt;&lt;T as Config&gt;::OnChargeTransaction as OnChargeTransaction&lt;T&gt;&gt;::Balance&gt;<div class=\"where\">where\n    &lt;T as Config&gt;::RuntimeCall: Dispatchable&lt;Info = DispatchInfo, PostInfo = PostDispatchInfo&gt;,</div></h4></section></summary><div class=\"docblock\"><p>Compute the actual post dispatch fee details for a particular transaction.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.length_to_fee\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">length_to_fee</a>(\n    length: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u32.html\">u32</a>\n) -&gt; &lt;&lt;T as Config&gt;::OnChargeTransaction as OnChargeTransaction&lt;T&gt;&gt;::Balance</h4></section></summary><div class=\"docblock\"><p>Compute the length portion of a fee by invoking the configured <code>LengthToFee</code> impl.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.weight_to_fee\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">weight_to_fee</a>(\n    weight: Weight\n) -&gt; &lt;&lt;T as Config&gt;::OnChargeTransaction as OnChargeTransaction&lt;T&gt;&gt;::Balance</h4></section></summary><div class=\"docblock\"><p>Compute the unadjusted portion of the weight fee by invoking the configured <code>WeightToFee</code>\nimpl. Note that the input <code>weight</code> is capped by the maximum block weight before computation.</p>\n</div></details></div></details>",0,"runtime_eden::TransactionPayment"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Hooks%3C%3C%3C%3CT+as+Config%3E::Block+as+HeaderProvider%3E::HeaderT+as+Header%3E::Number%3E-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-Hooks%3C%3C%3C%3CT+as+Config%3E::Block+as+HeaderProvider%3E::HeaderT+as+Header%3E::Number%3E-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; Hooks&lt;&lt;&lt;&lt;T as Config&gt;::Block as HeaderProvider&gt;::HeaderT as Header&gt;::Number&gt; for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.on_finalize\" class=\"method trait-impl\"><a href=\"#method.on_finalize\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">on_finalize</a>(\n    _: &lt;&lt;&lt;T as Config&gt;::Block as HeaderProvider&gt;::HeaderT as Header&gt;::Number\n)</h4></section></summary><div class='docblock'>Block finalization hook. This is called at the very end of block execution. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.integrity_test\" class=\"method trait-impl\"><a href=\"#method.integrity_test\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">integrity_test</a>()</h4></section></summary><div class='docblock'>Check the integrity of this pallet’s configuration. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.on_initialize\" class=\"method trait-impl\"><a href=\"#method.on_initialize\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">on_initialize</a>(_n: BlockNumber) -&gt; Weight</h4></section></summary><div class='docblock'>Block initialization hook. This is called at the very beginning of block execution. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.on_idle\" class=\"method trait-impl\"><a href=\"#method.on_idle\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">on_idle</a>(_n: BlockNumber, _remaining_weight: Weight) -&gt; Weight</h4></section></summary><div class='docblock'>Hook to consume a block’s idle time. This will run when the block is being finalized (before\n[<code>Hooks::on_finalize</code>]). <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.on_runtime_upgrade\" class=\"method trait-impl\"><a href=\"#method.on_runtime_upgrade\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">on_runtime_upgrade</a>() -&gt; Weight</h4></section></summary><div class='docblock'>Hook executed when a code change (aka. a “runtime upgrade”) is detected by FRAME. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.offchain_worker\" class=\"method trait-impl\"><a href=\"#method.offchain_worker\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">offchain_worker</a>(_n: BlockNumber)</h4></section></summary><div class='docblock'>Implementing this function on a pallet allows you to perform long-running tasks that are\ndispatched as separate threads, and entirely independent of the main wasm runtime. <a>Read more</a></div></details></div></details>","Hooks<<<<T as Config>::Block as HeaderProvider>::HeaderT as Header>::Number>","runtime_eden::TransactionPayment"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-Debug-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for Pallet&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, fmt: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.77.2/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.77.2/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.77.2/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.77.2/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","runtime_eden::TransactionPayment"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-OnInitialize%3C%3C%3C%3CT+as+Config%3E::Block+as+HeaderProvider%3E::HeaderT+as+Header%3E::Number%3E-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-OnInitialize%3C%3C%3C%3CT+as+Config%3E::Block+as+HeaderProvider%3E::HeaderT+as+Header%3E::Number%3E-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; OnInitialize&lt;&lt;&lt;&lt;T as Config&gt;::Block as HeaderProvider&gt;::HeaderT as Header&gt;::Number&gt; for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.on_initialize\" class=\"method trait-impl\"><a href=\"#method.on_initialize\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">on_initialize</a>(\n    n: &lt;&lt;&lt;T as Config&gt;::Block as HeaderProvider&gt;::HeaderT as Header&gt;::Number\n) -&gt; Weight</h4></section></summary><div class='docblock'>See [<code>Hooks::on_initialize</code>].</div></details></div></details>","OnInitialize<<<<T as Config>::Block as HeaderProvider>::HeaderT as Header>::Number>","runtime_eden::TransactionPayment"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PalletInfoAccess-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-PalletInfoAccess-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; PalletInfoAccess for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.index\" class=\"method trait-impl\"><a href=\"#method.index\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">index</a>() -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.usize.html\">usize</a></h4></section></summary><div class='docblock'>Index of the pallet as configured in the runtime.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.name\" class=\"method trait-impl\"><a href=\"#method.name\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">name</a>() -&gt; &amp;'static <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.str.html\">str</a></h4></section></summary><div class='docblock'>Name of the pallet as configured in the runtime.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.name_hash\" class=\"method trait-impl\"><a href=\"#method.name_hash\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">name_hash</a>() -&gt; [<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">16</a>]</h4></section></summary><div class='docblock'>Two128 hash of name.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.module_name\" class=\"method trait-impl\"><a href=\"#method.module_name\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">module_name</a>() -&gt; &amp;'static <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.str.html\">str</a></h4></section></summary><div class='docblock'>Name of the Rust module containing the pallet.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.crate_version\" class=\"method trait-impl\"><a href=\"#method.crate_version\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">crate_version</a>() -&gt; CrateVersion</h4></section></summary><div class='docblock'>Version of the crate containing the pallet.</div></details></div></details>","PalletInfoAccess","runtime_eden::TransactionPayment"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-IntegrityTest-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-IntegrityTest-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; IntegrityTest for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.integrity_test\" class=\"method trait-impl\"><a href=\"#method.integrity_test\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">integrity_test</a>()</h4></section></summary><div class='docblock'>See [<code>Hooks::integrity_test</code>].</div></details></div></details>","IntegrityTest","runtime_eden::TransactionPayment"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-OffchainWorker%3C%3C%3C%3CT+as+Config%3E::Block+as+HeaderProvider%3E::HeaderT+as+Header%3E::Number%3E-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-OffchainWorker%3C%3C%3C%3CT+as+Config%3E::Block+as+HeaderProvider%3E::HeaderT+as+Header%3E::Number%3E-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; OffchainWorker&lt;&lt;&lt;&lt;T as Config&gt;::Block as HeaderProvider&gt;::HeaderT as Header&gt;::Number&gt; for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.offchain_worker\" class=\"method trait-impl\"><a href=\"#method.offchain_worker\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">offchain_worker</a>(\n    n: &lt;&lt;&lt;T as Config&gt;::Block as HeaderProvider&gt;::HeaderT as Header&gt;::Number\n)</h4></section></summary><div class='docblock'>This function is being called after every block import (when fully synced). <a>Read more</a></div></details></div></details>","OffchainWorker<<<<T as Config>::Block as HeaderProvider>::HeaderT as Header>::Number>","runtime_eden::TransactionPayment"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-GetStorageVersion-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-GetStorageVersion-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; GetStorageVersion for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.CurrentStorageVersion\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.CurrentStorageVersion\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">CurrentStorageVersion</a> = NoStorageVersionSet</h4></section></summary><div class='docblock'>This will be filled out by the <a href=\"crate::pallet\"><code>pallet</code></a> macro. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.current_storage_version\" class=\"method trait-impl\"><a href=\"#method.current_storage_version\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">current_storage_version</a>(\n) -&gt; &lt;Pallet&lt;T&gt; as GetStorageVersion&gt;::CurrentStorageVersion</h4></section></summary><div class='docblock'>Returns the current storage version as supported by the pallet.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.on_chain_storage_version\" class=\"method trait-impl\"><a href=\"#method.on_chain_storage_version\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">on_chain_storage_version</a>() -&gt; StorageVersion</h4></section></summary><div class='docblock'>Returns the on-chain storage version of the pallet as stored in the storage.</div></details></div></details>","GetStorageVersion","runtime_eden::TransactionPayment"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-OnGenesis-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-OnGenesis-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; OnGenesis for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.on_genesis\" class=\"method trait-impl\"><a href=\"#method.on_genesis\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">on_genesis</a>()</h4></section></summary><div class='docblock'>Something that should happen at genesis.</div></details></div></details>","OnGenesis","runtime_eden::TransactionPayment"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PalletsInfoAccess-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-PalletsInfoAccess-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; PalletsInfoAccess for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.count\" class=\"method trait-impl\"><a href=\"#method.count\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">count</a>() -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.usize.html\">usize</a></h4></section></summary><div class='docblock'>The number of pallets’ information that this type represents. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.infos\" class=\"method trait-impl\"><a href=\"#method.infos\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">infos</a>() -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.77.2/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;PalletInfoData&gt;</h4></section></summary><div class='docblock'>All of the pallets’ information that this type represents.</div></details></div></details>","PalletsInfoAccess","runtime_eden::TransactionPayment"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-OnRuntimeUpgrade-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-OnRuntimeUpgrade-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; OnRuntimeUpgrade for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.on_runtime_upgrade\" class=\"method trait-impl\"><a href=\"#method.on_runtime_upgrade\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">on_runtime_upgrade</a>() -&gt; Weight</h4></section></summary><div class='docblock'>See [<code>Hooks::on_runtime_upgrade</code>].</div></details></div></details>","OnRuntimeUpgrade","runtime_eden::TransactionPayment"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Callable%3CT%3E-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-Callable%3CT%3E-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; Callable&lt;T&gt; for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><section id=\"associatedtype.RuntimeCall\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.RuntimeCall\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">RuntimeCall</a> = Call&lt;T&gt;</h4></section></div></details>","Callable<T>","runtime_eden::TransactionPayment"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-StorageInfoTrait-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-StorageInfoTrait-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; StorageInfoTrait for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><section id=\"method.storage_info\" class=\"method trait-impl\"><a href=\"#method.storage_info\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">storage_info</a>() -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.77.2/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;StorageInfo&gt;</h4></section></div></details>","StorageInfoTrait","runtime_eden::TransactionPayment"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-EstimateCallFee%3CAnyCall,+%3C%3CT+as+Config%3E::OnChargeTransaction+as+OnChargeTransaction%3CT%3E%3E::Balance%3E-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-EstimateCallFee%3CAnyCall,+%3C%3CT+as+Config%3E::OnChargeTransaction+as+OnChargeTransaction%3CT%3E%3E::Balance%3E-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, AnyCall&gt; EstimateCallFee&lt;AnyCall, &lt;&lt;T as Config&gt;::OnChargeTransaction as OnChargeTransaction&lt;T&gt;&gt;::Balance&gt; for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,\n    AnyCall: GetDispatchInfo + Encode,\n    &lt;T as Config&gt;::RuntimeCall: Dispatchable&lt;Info = DispatchInfo, PostInfo = PostDispatchInfo&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.estimate_call_fee\" class=\"method trait-impl\"><a href=\"#method.estimate_call_fee\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">estimate_call_fee</a>(\n    call: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.reference.html\">&amp;AnyCall</a>,\n    post_info: PostDispatchInfo\n) -&gt; &lt;&lt;T as Config&gt;::OnChargeTransaction as OnChargeTransaction&lt;T&gt;&gt;::Balance</h4></section></summary><div class='docblock'>Estimate the fee of this call. <a>Read more</a></div></details></div></details>","EstimateCallFee<AnyCall, <<T as Config>::OnChargeTransaction as OnChargeTransaction<T>>::Balance>","runtime_eden::TransactionPayment"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-OnFinalize%3C%3C%3C%3CT+as+Config%3E::Block+as+HeaderProvider%3E::HeaderT+as+Header%3E::Number%3E-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-OnFinalize%3C%3C%3C%3CT+as+Config%3E::Block+as+HeaderProvider%3E::HeaderT+as+Header%3E::Number%3E-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; OnFinalize&lt;&lt;&lt;&lt;T as Config&gt;::Block as HeaderProvider&gt;::HeaderT as Header&gt;::Number&gt; for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.on_finalize\" class=\"method trait-impl\"><a href=\"#method.on_finalize\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">on_finalize</a>(\n    n: &lt;&lt;&lt;T as Config&gt;::Block as HeaderProvider&gt;::HeaderT as Header&gt;::Number\n)</h4></section></summary><div class='docblock'>See [<code>Hooks::on_finalize</code>].</div></details></div></details>","OnFinalize<<<<T as Config>::Block as HeaderProvider>::HeaderT as Header>::Number>","runtime_eden::TransactionPayment"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Convert%3CWeight,+%3C%3CT+as+Config%3E::OnChargeTransaction+as+OnChargeTransaction%3CT%3E%3E::Balance%3E-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-Convert%3CWeight,+%3C%3CT+as+Config%3E::OnChargeTransaction+as+OnChargeTransaction%3CT%3E%3E::Balance%3E-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; Convert&lt;Weight, &lt;&lt;T as Config&gt;::OnChargeTransaction as OnChargeTransaction&lt;T&gt;&gt;::Balance&gt; for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.convert\" class=\"method trait-impl\"><a href=\"#method.convert\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">convert</a>(\n    weight: Weight\n) -&gt; &lt;&lt;T as Config&gt;::OnChargeTransaction as OnChargeTransaction&lt;T&gt;&gt;::Balance</h4></section></summary><div class=\"docblock\"><p>Compute the fee for the specified weight.</p>\n<p>This fee is already adjusted by the per block fee adjustment factor and is therefore the\nshare that the weight contributes to the overall fee of a transaction. It is mainly\nfor informational purposes and not used in the actual fee calculation.</p>\n</div></details></div></details>","Convert<Weight, <<T as Config>::OnChargeTransaction as OnChargeTransaction<T>>::Balance>","runtime_eden::TransactionPayment"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-PartialEq-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> for Pallet&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/cmp/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: &amp;Pallet&lt;T&gt;) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>self</code> and <code>other</code> values to be equal, and is used\nby <code>==</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.77.2/src/core/cmp.rs.html#242\">source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/cmp/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>!=</code>. The default implementation is almost always\nsufficient, and should not be overridden without very good reason.</div></details></div></details>","PartialEq","runtime_eden::TransactionPayment"],["<section id=\"impl-Eq-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-Eq-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> for Pallet&lt;T&gt;</h3></section>","Eq","runtime_eden::TransactionPayment"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-Clone-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for Pallet&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; Pallet&lt;T&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.77.2/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.77.2/src/core/clone.rs.html#169\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.77.2/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","runtime_eden::TransactionPayment"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-BeforeAllRuntimeMigrations-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-BeforeAllRuntimeMigrations-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; BeforeAllRuntimeMigrations for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.before_all_runtime_migrations\" class=\"method trait-impl\"><a href=\"#method.before_all_runtime_migrations\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">before_all_runtime_migrations</a>() -&gt; Weight</h4></section></summary><div class='docblock'>Something that should happen before runtime migrations are executed.</div></details></div></details>","BeforeAllRuntimeMigrations","runtime_eden::TransactionPayment"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-OnIdle%3C%3C%3C%3CT+as+Config%3E::Block+as+HeaderProvider%3E::HeaderT+as+Header%3E::Number%3E-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-OnIdle%3C%3C%3C%3CT+as+Config%3E::Block+as+HeaderProvider%3E::HeaderT+as+Header%3E::Number%3E-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; OnIdle&lt;&lt;&lt;&lt;T as Config&gt;::Block as HeaderProvider&gt;::HeaderT as Header&gt;::Number&gt; for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.on_idle\" class=\"method trait-impl\"><a href=\"#method.on_idle\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">on_idle</a>(\n    n: &lt;&lt;&lt;T as Config&gt;::Block as HeaderProvider&gt;::HeaderT as Header&gt;::Number,\n    remaining_weight: Weight\n) -&gt; Weight</h4></section></summary><div class='docblock'>See [<code>Hooks::on_idle</code>].</div></details></div></details>","OnIdle<<<<T as Config>::Block as HeaderProvider>::HeaderT as Header>::Number>","runtime_eden::TransactionPayment"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-WhitelistedStorageKeys-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-WhitelistedStorageKeys-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; WhitelistedStorageKeys for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.whitelisted_storage_keys\" class=\"method trait-impl\"><a href=\"#method.whitelisted_storage_keys\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">whitelisted_storage_keys</a>() -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.77.2/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;TrackedStorageKey&gt;</h4></section></summary><div class='docblock'>Returns a <a href=\"https://doc.rust-lang.org/1.77.2/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\"><code>Vec&lt;TrackedStorageKey&gt;</code></a> indicating the storage keys that\nshould be whitelisted during benchmarking. This means that those keys\nwill be excluded from the benchmarking performance calculation.</div></details></div></details>","WhitelistedStorageKeys","runtime_eden::TransactionPayment"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()