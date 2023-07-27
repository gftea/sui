(function() {var implementors = {
"sui_indexer":[["impl&lt;__DB: Backend&gt; QueryableByName&lt;__DB&gt; for <a class=\"struct\" href=\"sui_indexer/models/transactions/struct.Transaction.html\" title=\"struct sui_indexer::models::transactions::Transaction\">Transaction</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i64.html\">i64</a>: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/transactions/columns/struct.id.html\" title=\"struct sui_indexer::schema::transactions::columns::id\">id</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/transactions/columns/struct.transaction_count.html\" title=\"struct sui_indexer::schema::transactions::columns::transaction_count\">transaction_count</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/transactions/columns/struct.gas_object_sequence.html\" title=\"struct sui_indexer::schema::transactions::columns::gas_object_sequence\">gas_object_sequence</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/transactions/columns/struct.gas_budget.html\" title=\"struct sui_indexer::schema::transactions::columns::gas_budget\">gas_budget</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/transactions/columns/struct.total_gas_cost.html\" title=\"struct sui_indexer::schema::transactions::columns::total_gas_cost\">total_gas_cost</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/transactions/columns/struct.computation_cost.html\" title=\"struct sui_indexer::schema::transactions::columns::computation_cost\">computation_cost</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/transactions/columns/struct.storage_cost.html\" title=\"struct sui_indexer::schema::transactions::columns::storage_cost\">storage_cost</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/transactions/columns/struct.storage_rebate.html\" title=\"struct sui_indexer::schema::transactions::columns::storage_rebate\">storage_rebate</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/transactions/columns/struct.non_refundable_storage_fee.html\" title=\"struct sui_indexer::schema::transactions::columns::non_refundable_storage_fee\">non_refundable_storage_fee</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/transactions/columns/struct.gas_price.html\" title=\"struct sui_indexer::schema::transactions::columns::gas_price\">gas_price</a>&gt;, __DB&gt;,\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/transactions/columns/struct.transaction_digest.html\" title=\"struct sui_indexer::schema::transactions::columns::transaction_digest\">transaction_digest</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/transactions/columns/struct.sender.html\" title=\"struct sui_indexer::schema::transactions::columns::sender\">sender</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/transactions/columns/struct.transaction_kind.html\" title=\"struct sui_indexer::schema::transactions::columns::transaction_kind\">transaction_kind</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/transactions/columns/struct.gas_object_id.html\" title=\"struct sui_indexer::schema::transactions::columns::gas_object_id\">gas_object_id</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/transactions/columns/struct.gas_object_digest.html\" title=\"struct sui_indexer::schema::transactions::columns::gas_object_digest\">gas_object_digest</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/transactions/columns/struct.transaction_effects_content.html\" title=\"struct sui_indexer::schema::transactions::columns::transaction_effects_content\">transaction_effects_content</a>&gt;, __DB&gt;,\n    <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i64.html\">i64</a>&gt;: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/transactions/columns/struct.checkpoint_sequence_number.html\" title=\"struct sui_indexer::schema::transactions::columns::checkpoint_sequence_number\">checkpoint_sequence_number</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/transactions/columns/struct.timestamp_ms.html\" title=\"struct sui_indexer::schema::transactions::columns::timestamp_ms\">timestamp_ms</a>&gt;, __DB&gt;,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a>: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/transactions/columns/struct.execution_success.html\" title=\"struct sui_indexer::schema::transactions::columns::execution_success\">execution_success</a>&gt;, __DB&gt;,\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt;: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/transactions/columns/struct.raw_transaction.html\" title=\"struct sui_indexer::schema::transactions::columns::raw_transaction\">raw_transaction</a>&gt;, __DB&gt;,\n    <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a>&gt;: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/transactions/columns/struct.confirmed_local_execution.html\" title=\"struct sui_indexer::schema::transactions::columns::confirmed_local_execution\">confirmed_local_execution</a>&gt;, __DB&gt;,</span>"],["impl&lt;__DB: Backend&gt; QueryableByName&lt;__DB&gt; for <a class=\"struct\" href=\"sui_indexer/models/addresses/struct.DBAddressStats.html\" title=\"struct sui_indexer::models::addresses::DBAddressStats\">DBAddressStats</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i64.html\">i64</a>: FromSql&lt;BigInt, __DB&gt;,</span>"],["impl&lt;__DB: Backend&gt; QueryableByName&lt;__DB&gt; for <a class=\"struct\" href=\"sui_indexer/models/objects/struct.Object.html\" title=\"struct sui_indexer::models::objects::Object\">Object</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i64.html\">i64</a>: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/objects/columns/struct.epoch.html\" title=\"struct sui_indexer::schema::objects::columns::epoch\">epoch</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/objects/columns/struct.checkpoint.html\" title=\"struct sui_indexer::schema::objects::columns::checkpoint\">checkpoint</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/objects/columns/struct.version.html\" title=\"struct sui_indexer::schema::objects::columns::version\">version</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/objects/columns/struct.storage_rebate.html\" title=\"struct sui_indexer::schema::objects::columns::storage_rebate\">storage_rebate</a>&gt;, __DB&gt;,\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/objects/columns/struct.object_id.html\" title=\"struct sui_indexer::schema::objects::columns::object_id\">object_id</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/objects/columns/struct.object_digest.html\" title=\"struct sui_indexer::schema::objects::columns::object_digest\">object_digest</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/objects/columns/struct.previous_transaction.html\" title=\"struct sui_indexer::schema::objects::columns::previous_transaction\">previous_transaction</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/objects/columns/struct.object_type.html\" title=\"struct sui_indexer::schema::objects::columns::object_type\">object_type</a>&gt;, __DB&gt;,\n    <a class=\"enum\" href=\"sui_indexer/models/owners/enum.OwnerType.html\" title=\"enum sui_indexer::models::owners::OwnerType\">OwnerType</a>: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/objects/columns/struct.owner_type.html\" title=\"struct sui_indexer::schema::objects::columns::owner_type\">owner_type</a>&gt;, __DB&gt;,\n    <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt;: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/objects/columns/struct.owner_address.html\" title=\"struct sui_indexer::schema::objects::columns::owner_address\">owner_address</a>&gt;, __DB&gt;,\n    <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i64.html\">i64</a>&gt;: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/objects/columns/struct.initial_shared_version.html\" title=\"struct sui_indexer::schema::objects::columns::initial_shared_version\">initial_shared_version</a>&gt;, __DB&gt;,\n    <a class=\"enum\" href=\"sui_indexer/models/objects/enum.ObjectStatus.html\" title=\"enum sui_indexer::models::objects::ObjectStatus\">ObjectStatus</a>: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/objects/columns/struct.object_status.html\" title=\"struct sui_indexer::schema::objects::columns::object_status\">object_status</a>&gt;, __DB&gt;,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a>: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/objects/columns/struct.has_public_transfer.html\" title=\"struct sui_indexer::schema::objects::columns::has_public_transfer\">has_public_transfer</a>&gt;, __DB&gt;,\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"struct\" href=\"sui_indexer/models/objects/struct.NamedBcsBytes.html\" title=\"struct sui_indexer::models::objects::NamedBcsBytes\">NamedBcsBytes</a>&gt;: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/objects/columns/struct.bcs.html\" title=\"struct sui_indexer::schema::objects::columns::bcs\">bcs</a>&gt;, __DB&gt;,</span>"],["impl&lt;__DB: Backend&gt; QueryableByName&lt;__DB&gt; for <a class=\"struct\" href=\"sui_indexer/models/network_metrics/struct.DBNetworkMetrics.html\" title=\"struct sui_indexer::models::network_metrics::DBNetworkMetrics\">DBNetworkMetrics</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.f64.html\">f64</a>: FromSql&lt;Double, __DB&gt;,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i64.html\">i64</a>: FromSql&lt;BigInt, __DB&gt;,</span>"],["impl&lt;__DB: Backend&gt; QueryableByName&lt;__DB&gt; for <a class=\"struct\" href=\"sui_indexer/models/checkpoint_metrics/struct.Tps.html\" title=\"struct sui_indexer::models::checkpoint_metrics::Tps\">Tps</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.f64.html\">f64</a>: FromSql&lt;Float8, __DB&gt;,</span>"],["impl&lt;__DB: Backend&gt; QueryableByName&lt;__DB&gt; for <a class=\"struct\" href=\"sui_indexer/models/network_metrics/struct.DBMoveCallMetrics.html\" title=\"struct sui_indexer::models::network_metrics::DBMoveCallMetrics\">DBMoveCallMetrics</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i64.html\">i64</a>: FromSql&lt;BigInt, __DB&gt;,\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>: FromSql&lt;Text, __DB&gt;,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()