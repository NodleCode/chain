(function() {var type_impls = {
"runtime_eden":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.set\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">set</a>(\n    origin: &lt;T as Config&gt;::RuntimeOrigin,\n    now: &lt;T as Config&gt;::Moment\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.76.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.unit.html\">()</a>, DispatchError&gt;</h4></section></summary><div class=\"docblock\"><p>Set the current time.</p>\n<p>This call should be invoked exactly once per block. It will panic at the finalization\nphase, if this call hasn’t been invoked by that time.</p>\n<p>The timestamp should be greater than the previous one by the amount specified by\n<code>MinimumPeriod</code>.</p>\n<p>The dispatch origin for this call must be <code>Inherent</code>.</p>\n<h6 id=\"complexity\"><a href=\"#complexity\">Complexity</a></h6>\n<ul>\n<li><code>O(1)</code> (Note that implementations of <code>OnTimestampSet</code> must also be <code>O(1)</code>)</li>\n<li>1 storage read and 1 storage mutation (codec <code>O(1)</code>). (because of <code>DidUpdate::take</code> in\n<code>on_finalize</code>)</li>\n<li>1 event handler <code>on_timestamp_set</code>. Must be <code>O(1)</code>.</li>\n</ul>\n</div></details></div></details>",0,"runtime_eden::Timestamp"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.now\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">now</a>() -&gt; &lt;T as Config&gt;::Moment</h4></section></summary><div class=\"docblock\"><p>An auto-generated getter for [<code>Now</code>].</p>\n</div></details></div></details>",0,"runtime_eden::Timestamp"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.get\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">get</a>() -&gt; &lt;T as Config&gt;::Moment</h4></section></summary><div class=\"docblock\"><p>Get the current time for the current block.</p>\n<p>NOTE: if this function is called prior to setting the timestamp,\nit will return the timestamp of the previous block.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.set_timestamp\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">set_timestamp</a>(now: &lt;T as Config&gt;::Moment)</h4></section></summary><div class=\"docblock\"><p>Set the timestamp to something in particular. Only used for tests.</p>\n</div></details></div></details>",0,"runtime_eden::Timestamp"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PalletsInfoAccess-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-PalletsInfoAccess-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; PalletsInfoAccess for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.count\" class=\"method trait-impl\"><a href=\"#method.count\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">count</a>() -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.usize.html\">usize</a></h4></section></summary><div class='docblock'>The number of pallets’ information that this type represents. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.infos\" class=\"method trait-impl\"><a href=\"#method.infos\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">infos</a>() -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.76.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;PalletInfoData&gt;</h4></section></summary><div class='docblock'>All of the pallets’ information that this type represents.</div></details></div></details>","PalletsInfoAccess","runtime_eden::Timestamp"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-GetStorageVersion-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-GetStorageVersion-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; GetStorageVersion for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.CurrentStorageVersion\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.CurrentStorageVersion\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">CurrentStorageVersion</a> = NoStorageVersionSet</h4></section></summary><div class='docblock'>This will be filled out by the <a href=\"crate::pallet\"><code>pallet</code></a> macro. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.current_storage_version\" class=\"method trait-impl\"><a href=\"#method.current_storage_version\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">current_storage_version</a>(\n) -&gt; &lt;Pallet&lt;T&gt; as GetStorageVersion&gt;::CurrentStorageVersion</h4></section></summary><div class='docblock'>Returns the current storage version as supported by the pallet.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.on_chain_storage_version\" class=\"method trait-impl\"><a href=\"#method.on_chain_storage_version\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">on_chain_storage_version</a>() -&gt; StorageVersion</h4></section></summary><div class='docblock'>Returns the on-chain storage version of the pallet as stored in the storage.</div></details></div></details>","GetStorageVersion","runtime_eden::Timestamp"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-StorageInfoTrait-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-StorageInfoTrait-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; StorageInfoTrait for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><section id=\"method.storage_info\" class=\"method trait-impl\"><a href=\"#method.storage_info\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">storage_info</a>() -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.76.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;StorageInfo&gt;</h4></section></div></details>","StorageInfoTrait","runtime_eden::Timestamp"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-OnIdle%3C%3C%3C%3CT+as+Config%3E::Block+as+HeaderProvider%3E::HeaderT+as+HeaderT%3E::Number%3E-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-OnIdle%3C%3C%3C%3CT+as+Config%3E::Block+as+HeaderProvider%3E::HeaderT+as+HeaderT%3E::Number%3E-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; OnIdle&lt;&lt;&lt;&lt;T as Config&gt;::Block as HeaderProvider&gt;::HeaderT as Header&gt;::Number&gt; for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.on_idle\" class=\"method trait-impl\"><a href=\"#method.on_idle\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">on_idle</a>(\n    n: &lt;&lt;&lt;T as Config&gt;::Block as HeaderProvider&gt;::HeaderT as Header&gt;::Number,\n    remaining_weight: Weight\n) -&gt; Weight</h4></section></summary><div class='docblock'>The block is being finalized.\nImplement to have something happen in case there is leftover weight.\nCheck the passed <code>remaining_weight</code> to make sure it is high enough to allow for\nyour pallet’s extra computation. <a>Read more</a></div></details></div></details>","OnIdle<<<<T as Config>::Block as HeaderProvider>::HeaderT as Header>::Number>","runtime_eden::Timestamp"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-Debug-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for Pallet&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.76.0/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, fmt: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.76.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.76.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.76.0/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.76.0/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","runtime_eden::Timestamp"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PalletInfoAccess-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-PalletInfoAccess-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; PalletInfoAccess for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.index\" class=\"method trait-impl\"><a href=\"#method.index\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">index</a>() -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.usize.html\">usize</a></h4></section></summary><div class='docblock'>Index of the pallet as configured in the runtime.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.name\" class=\"method trait-impl\"><a href=\"#method.name\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">name</a>() -&gt; &amp;'static <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.str.html\">str</a></h4></section></summary><div class='docblock'>Name of the pallet as configured in the runtime.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.module_name\" class=\"method trait-impl\"><a href=\"#method.module_name\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">module_name</a>() -&gt; &amp;'static <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.str.html\">str</a></h4></section></summary><div class='docblock'>Name of the Rust module containing the pallet.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.crate_version\" class=\"method trait-impl\"><a href=\"#method.crate_version\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">crate_version</a>() -&gt; CrateVersion</h4></section></summary><div class='docblock'>Version of the crate containing the pallet.</div></details></div></details>","PalletInfoAccess","runtime_eden::Timestamp"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-ProvideInherent-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-ProvideInherent-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; ProvideInherent for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Call\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Call\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">Call</a> = Call&lt;T&gt;</h4></section></summary><div class='docblock'>The call type of the pallet.</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.Error\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Error\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">Error</a> = InherentError</h4></section></summary><div class='docblock'>The error returned by <code>check_inherent</code>.</div></details><details class=\"toggle\" open><summary><section id=\"associatedconstant.INHERENT_IDENTIFIER\" class=\"associatedconstant trait-impl\"><a href=\"#associatedconstant.INHERENT_IDENTIFIER\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a class=\"constant\">INHERENT_IDENTIFIER</a>: [<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.array.html\">8</a>] = INHERENT_IDENTIFIER</h4></section></summary><div class='docblock'>The inherent identifier used by this inherent.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.create_inherent\" class=\"method trait-impl\"><a href=\"#method.create_inherent\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">create_inherent</a>(\n    data: &amp;InherentData\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.76.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;&lt;Pallet&lt;T&gt; as ProvideInherent&gt;::Call&gt;</h4></section></summary><div class='docblock'>Create an inherent out of the given <code>InherentData</code>. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.check_inherent\" class=\"method trait-impl\"><a href=\"#method.check_inherent\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">check_inherent</a>(\n    call: &amp;&lt;Pallet&lt;T&gt; as ProvideInherent&gt;::Call,\n    data: &amp;InherentData\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.76.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.unit.html\">()</a>, &lt;Pallet&lt;T&gt; as ProvideInherent&gt;::Error&gt;</h4></section></summary><div class='docblock'>Check whether the given inherent is valid. Checking the inherent is optional and can be\nomitted by using the default implementation. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.is_inherent\" class=\"method trait-impl\"><a href=\"#method.is_inherent\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">is_inherent</a>(call: &amp;&lt;Pallet&lt;T&gt; as ProvideInherent&gt;::Call) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Return whether the call is an inherent call. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.is_inherent_required\" class=\"method trait-impl\"><a href=\"#method.is_inherent_required\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">is_inherent_required</a>(\n    _: &amp;InherentData\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.76.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/1.76.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;Self::Error&gt;, Self::Error&gt;</h4></section></summary><div class='docblock'>Determines whether this inherent is required in this block. <a>Read more</a></div></details></div></details>","ProvideInherent","runtime_eden::Timestamp"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Callable%3CT%3E-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-Callable%3CT%3E-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; Callable&lt;T&gt; for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><section id=\"associatedtype.RuntimeCall\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.RuntimeCall\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">RuntimeCall</a> = Call&lt;T&gt;</h4></section></div></details>","Callable<T>","runtime_eden::Timestamp"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-OffchainWorker%3C%3C%3C%3CT+as+Config%3E::Block+as+HeaderProvider%3E::HeaderT+as+HeaderT%3E::Number%3E-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-OffchainWorker%3C%3C%3C%3CT+as+Config%3E::Block+as+HeaderProvider%3E::HeaderT+as+HeaderT%3E::Number%3E-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; OffchainWorker&lt;&lt;&lt;&lt;T as Config&gt;::Block as HeaderProvider&gt;::HeaderT as Header&gt;::Number&gt; for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.offchain_worker\" class=\"method trait-impl\"><a href=\"#method.offchain_worker\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">offchain_worker</a>(\n    n: &lt;&lt;&lt;T as Config&gt;::Block as HeaderProvider&gt;::HeaderT as Header&gt;::Number\n)</h4></section></summary><div class='docblock'>This function is being called after every block import (when fully synced). <a>Read more</a></div></details></div></details>","OffchainWorker<<<<T as Config>::Block as HeaderProvider>::HeaderT as Header>::Number>","runtime_eden::Timestamp"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-UnixTime-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-UnixTime-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; UnixTime for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"docblock\"><p>Before the timestamp inherent is applied, it returns the time of previous block.</p>\n<p>On genesis the time returned is not valid.</p>\n</div><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.now\" class=\"method trait-impl\"><a href=\"#method.now\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">now</a>() -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.76.0/core/time/struct.Duration.html\" title=\"struct core::time::Duration\">Duration</a></h4></section></summary><div class='docblock'>Return duration since <code>SystemTime::UNIX_EPOCH</code>.</div></details></div></details>","UnixTime","runtime_eden::Timestamp"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Time-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-Time-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; Time for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.now\" class=\"method trait-impl\"><a href=\"#method.now\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">now</a>() -&gt; &lt;Pallet&lt;T&gt; as Time&gt;::Moment</h4></section></summary><div class=\"docblock\"><p>Before the first set of now with inherent the value returned is zero.</p>\n</div></details><section id=\"associatedtype.Moment\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Moment\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">Moment</a> = &lt;T as Config&gt;::Moment</h4></section></div></details>","Time","runtime_eden::Timestamp"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-WhitelistedStorageKeys-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-WhitelistedStorageKeys-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; WhitelistedStorageKeys for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.whitelisted_storage_keys\" class=\"method trait-impl\"><a href=\"#method.whitelisted_storage_keys\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">whitelisted_storage_keys</a>() -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.76.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;TrackedStorageKey&gt;</h4></section></summary><div class='docblock'>Returns a <a href=\"https://doc.rust-lang.org/1.76.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\"><code>Vec&lt;TrackedStorageKey&gt;</code></a> indicating the storage keys that\nshould be whitelisted during benchmarking. This means that those keys\nwill be excluded from the benchmarking performance calculation.</div></details></div></details>","WhitelistedStorageKeys","runtime_eden::Timestamp"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-OnFinalize%3C%3C%3C%3CT+as+Config%3E::Block+as+HeaderProvider%3E::HeaderT+as+HeaderT%3E::Number%3E-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-OnFinalize%3C%3C%3C%3CT+as+Config%3E::Block+as+HeaderProvider%3E::HeaderT+as+HeaderT%3E::Number%3E-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; OnFinalize&lt;&lt;&lt;&lt;T as Config&gt;::Block as HeaderProvider&gt;::HeaderT as Header&gt;::Number&gt; for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.on_finalize\" class=\"method trait-impl\"><a href=\"#method.on_finalize\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">on_finalize</a>(\n    n: &lt;&lt;&lt;T as Config&gt;::Block as HeaderProvider&gt;::HeaderT as Header&gt;::Number\n)</h4></section></summary><div class='docblock'>The block is being finalized. Implement to have something happen. <a>Read more</a></div></details></div></details>","OnFinalize<<<<T as Config>::Block as HeaderProvider>::HeaderT as Header>::Number>","runtime_eden::Timestamp"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-IntegrityTest-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-IntegrityTest-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; IntegrityTest for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.integrity_test\" class=\"method trait-impl\"><a href=\"#method.integrity_test\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">integrity_test</a>()</h4></section></summary><div class='docblock'>Run integrity test. <a>Read more</a></div></details></div></details>","IntegrityTest","runtime_eden::Timestamp"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-OnGenesis-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-OnGenesis-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; OnGenesis for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.on_genesis\" class=\"method trait-impl\"><a href=\"#method.on_genesis\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">on_genesis</a>()</h4></section></summary><div class='docblock'>Something that should happen at genesis.</div></details></div></details>","OnGenesis","runtime_eden::Timestamp"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-PartialEq-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> for Pallet&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.76.0/core/cmp/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: &amp;Pallet&lt;T&gt;) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>self</code> and <code>other</code> values to be equal, and is used\nby <code>==</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.76.0/src/core/cmp.rs.html#242\">source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.76.0/core/cmp/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>!=</code>. The default implementation is almost always\nsufficient, and should not be overridden without very good reason.</div></details></div></details>","PartialEq","runtime_eden::Timestamp"],["<section id=\"impl-Eq-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-Eq-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> for Pallet&lt;T&gt;</h3></section>","Eq","runtime_eden::Timestamp"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-Clone-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for Pallet&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.76.0/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; Pallet&lt;T&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.76.0/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.76.0/src/core/clone.rs.html#169\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.76.0/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.76.0/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","runtime_eden::Timestamp"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-OnRuntimeUpgrade-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-OnRuntimeUpgrade-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; OnRuntimeUpgrade for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.on_runtime_upgrade\" class=\"method trait-impl\"><a href=\"#method.on_runtime_upgrade\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">on_runtime_upgrade</a>() -&gt; Weight</h4></section></summary><div class='docblock'>Perform a module upgrade. <a>Read more</a></div></details></div></details>","OnRuntimeUpgrade","runtime_eden::Timestamp"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Hooks%3C%3C%3C%3CT+as+Config%3E::Block+as+HeaderProvider%3E::HeaderT+as+HeaderT%3E::Number%3E-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-Hooks%3C%3C%3C%3CT+as+Config%3E::Block+as+HeaderProvider%3E::HeaderT+as+HeaderT%3E::Number%3E-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; Hooks&lt;&lt;&lt;&lt;T as Config&gt;::Block as HeaderProvider&gt;::HeaderT as Header&gt;::Number&gt; for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.on_initialize\" class=\"method trait-impl\"><a href=\"#method.on_initialize\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">on_initialize</a>(\n    _n: &lt;&lt;&lt;T as Config&gt;::Block as HeaderProvider&gt;::HeaderT as Header&gt;::Number\n) -&gt; Weight</h4></section></summary><div class=\"docblock\"><p>dummy <code>on_initialize</code> to return the weight used in <code>on_finalize</code>.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.on_finalize\" class=\"method trait-impl\"><a href=\"#method.on_finalize\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">on_finalize</a>(\n    _n: &lt;&lt;&lt;T as Config&gt;::Block as HeaderProvider&gt;::HeaderT as Header&gt;::Number\n)</h4></section></summary><div class=\"docblock\"><h6 id=\"complexity\"><a href=\"#complexity\">Complexity</a></h6>\n<ul>\n<li><code>O(1)</code></li>\n</ul>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.on_idle\" class=\"method trait-impl\"><a href=\"#method.on_idle\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">on_idle</a>(_n: BlockNumber, _remaining_weight: Weight) -&gt; Weight</h4></section></summary><div class='docblock'>This will be run when the block is being finalized (before <code>on_finalize</code>). <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.on_runtime_upgrade\" class=\"method trait-impl\"><a href=\"#method.on_runtime_upgrade\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">on_runtime_upgrade</a>() -&gt; Weight</h4></section></summary><div class='docblock'>Perform a module upgrade. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.offchain_worker\" class=\"method trait-impl\"><a href=\"#method.offchain_worker\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">offchain_worker</a>(_n: BlockNumber)</h4></section></summary><div class='docblock'>Implementing this function on a module allows you to perform long-running tasks\nthat make (by default) validators generate transactions that feed results\nof those long-running computations back on chain. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.integrity_test\" class=\"method trait-impl\"><a href=\"#method.integrity_test\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">integrity_test</a>()</h4></section></summary><div class='docblock'>Run integrity test. <a>Read more</a></div></details></div></details>","Hooks<<<<T as Config>::Block as HeaderProvider>::HeaderT as Header>::Number>","runtime_eden::Timestamp"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-OnInitialize%3C%3C%3C%3CT+as+Config%3E::Block+as+HeaderProvider%3E::HeaderT+as+HeaderT%3E::Number%3E-for-Pallet%3CT%3E\" class=\"impl\"><a href=\"#impl-OnInitialize%3C%3C%3C%3CT+as+Config%3E::Block+as+HeaderProvider%3E::HeaderT+as+HeaderT%3E::Number%3E-for-Pallet%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; OnInitialize&lt;&lt;&lt;&lt;T as Config&gt;::Block as HeaderProvider&gt;::HeaderT as Header&gt;::Number&gt; for Pallet&lt;T&gt;<div class=\"where\">where\n    T: Config,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.on_initialize\" class=\"method trait-impl\"><a href=\"#method.on_initialize\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">on_initialize</a>(\n    n: &lt;&lt;&lt;T as Config&gt;::Block as HeaderProvider&gt;::HeaderT as Header&gt;::Number\n) -&gt; Weight</h4></section></summary><div class='docblock'>The block is being initialized. Implement to have something happen. <a>Read more</a></div></details></div></details>","OnInitialize<<<<T as Config>::Block as HeaderProvider>::HeaderT as Header>::Number>","runtime_eden::Timestamp"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()