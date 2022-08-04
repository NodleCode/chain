
## UC-01 | Relaychain to Parachain

### Relay Chain Side

**genesis**

| Acc | balances |
| --- | --- |
| AccountId::from(ALICE) | dot(100f64) |
| ParaId::from(2026 as u32).into_account() | dot(100f64) |

**Extrincis Flow**

```plantuml
@startuml
autonumber "<b>[000]"
skinparam monochrome true

participant "Test" as test
participant "XcmPallet" as pxcm
participant "XcmExecutor" as pxcmexec
participant "XcmSender" as xcmsender

note over test

Alice :: 100 dot
Para id :: 100 dot

end note

test->pxcm: reserve_transfer_assets()
activate pxcm

note left pxcm
reserve_transfer_assets(
	polkadot_runtime::Origin::signed(ALICE.into()),
	Box::new(VersionedMultiLocation::V1(X1(Parachain(2026)).into())),
	Box::new(VersionedMultiLocation::V1(
		X1(Junction::AccountId32 {
			id: BOB,
			network: NetworkId::Any
		})
		.into()
	)),
	Box::new(VersionedMultiAssets::V1((Here, dot(1f64)).into())),
	0,
)
end note

' Construct the XCM message & Forward it
note over pxcm
let xcm = Xcm(vec![
	<b>BuyExecution { fees, weight_limit },
	<b>DepositAsset { assets: Wild(All), max_assets, beneficiary },
]);
let mut message = Xcm(
	vec![<b>TransferReserveAsset { assets, dest, xcm }]
);
let weight = T::Weigher::weight(
	&mut message
).map_err(|()| Error::<T>::UnweighableMessage)?;
end note

' Call the executor
pxcm->pxcmexec: execute_xcm_in_credit()
activate pxcmexec

note left pxcmexec
execute_xcm_in_credit(
	origin_location,
	message,
	weight,
	weight
)
end note

pxcmexec->pxcmexec: "Event::Transfer()"

pxcmexec->xcmsender: send_xcm()
activate xcmsender

note left xcmsender
send_xcm(
	dest,
	Xcm(message)
).map_err(Into::into)
end note

xcmsender->pxcmexec
deactivate xcmsender

pxcmexec->pxcm:
deactivate pxcmexec

pxcm->pxcm: "Event::Attempted()"

pxcm->test:
deactivate pxcm

note over test

Alice :: 99 dot
Para id :: 101 dot

end note

@enduml
```

### parachain Side | NodleNet | 2026

**genesis**

```plantuml
@startuml
autonumber "<b>[000]"
skinparam monochrome true

participant "Test" as test
participant "assets" as asset

' force_create()

test->asset: force_create()
activate asset

note left asset
Assets::force_create(
	Origin::root(),
	DOT,
	MultiAddress::Id(AccountId::from(ALICE)),
	true,
	1,
)
end note

asset->test:
deactivate asset

' force_set_metadata()

test->asset: force_set_metadata()
activate asset

note left asset
Assets::force_set_metadata(
	Origin::root(),
	DOT,
	b"Polkadot".to_vec(),
	b"DOT".to_vec(),
	12,
	false,
)
end note

asset->test:
deactivate asset

' mint()

test->asset: mint()
activate asset

note left asset
Assets::mint(
	Origin::signed(AccountId::from(ALICE)),
	DOT,
	MultiAddress::Id(AccountId::from(ALICE)),
	dot(100f64),
)
end note

asset->test:
deactivate asset

@enduml
```

**dmpqueue execution flow**

```plantuml
@startuml
autonumber "<b>[000]"
skinparam monochrome true

participant "frame-executive" as executive
participant "dmp-queue" as dmpq
participant "dmp-queue-impl" as dmpqimpl
participant "XcmExecutor" as pxcmexec
participant "traits-MultiCurrencyAdapter" as mcadopt
participant "frame-system" as system
participant "assets" as asset

note over dmpq

Alice :: 100 dot
Bob :: 0 dot

end note


executive->dmpq: on_idle()
activate dmpq

note left dmpq

fn on_idle(
	_now: T::BlockNumber,
	max_weight: Weight
)

Self::service_queue(
	max_weight
)
end note

dmpq->dmpqimpl: try_service_message()
activate dmpqimpl

dmpqimpl->pxcmexec: execute_xcm()
activate pxcmexec

note left pxcmexec
T::XcmExecutor::execute_xcm(
	Parent,
	maybe_msg,
	limit
);
end note

pxcmexec->mcadopt: deposit_asset()
activate mcadopt

' asset trans-01

mcadopt->system: on_created_account()
activate system

note over system
Event::NewAccount()
end note

system->mcadopt
deactivate system

mcadopt->asset: do_mint()
activate asset

note over asset
Event::Issued()
To Bob :: 0.9999904000 dot
end note

asset->mcadopt
deactivate asset

' asset trans-02

mcadopt->system: on_created_account()
activate system

note over system
Event::NewAccount()
end note

system->mcadopt
deactivate system

mcadopt->asset: do_mint()
activate asset

note over asset
Event::Issued()
To Fee :: 0.0000096000 dot
end note

asset->mcadopt
deactivate asset

mcadopt->pxcmexec:
deactivate mcadopt

pxcmexec->dmpqimpl
deactivate pxcmexec

note over dmpqimpl
Event::ExecutedDownward()
end note

dmpqimpl->dmpq
deactivate dmpqimpl

dmpq->executive:
deactivate executive

note over dmpq

Alice :: 100 dot
Bob :: 0.9999904000 dot

end note

@enduml
```

## UC-02 |  Parachain to Relachain

## Relay Chain Side

```plantuml
@startuml
autonumber "<b>[000]"
skinparam monochrome true

participant "Test" as test
participant "xcm-emulator" as xcmemul
participant "Ump" as ump
participant "XcmExecutor" as xcmexec

note over test

Alice :: 100 dot
Bob :: 0 dot
Para id :: 100 dot

end note

test->xcmemul: _process_upward_messages()
activate xcmemul

' looping over the UPWARD_MESSAGES

note over xcmemul
while let Some((from_para_id, msg)) = $crate::UPWARD_MESSAGES.with(|b| b.borrow_mut().pop_front()) {
	let _ =  <$relay_chain>::process_upward_message(
		from_para_id.into(),
		&msg[..],
		$crate::Weight::max_value(),
	);
}
end note


xcmemul->ump: process_upward_message()
activate ump

ump->xcmexec: execute_xcm()
activate xcmexec

note over xcmexec

Event::Withdraw()
Event::Deposit()
Event::NewAccount()
Event::Endowed()

end note

xcmexec->ump
deactivate xcmexec

note over ump
Event::ExecutedUpward()
end note

ump->xcmemul
deactivate ump

xcmemul->test
deactivate xcmemul

note over test

Alice :: 100 dot
Bob :: 9.9530582548 dot
Para id :: 90 dot

end note

@enduml
```

## parachain Side | NodleNet | 2026


```plantuml
@startuml
autonumber "<b>[000]"
skinparam monochrome true

participant "Test" as test
participant "orml_xtokens" as xtokens
participant "orml_xtokens impl" as xtokensimpl
participant "XcmExecutor" as xcmexec
participant "vm.exec" as vmexec
participant "trait_xcm" as txcm
participant "XcmSender" as xcmSender

' XTokens::transfer()

note over test

Alice :: 100 dot
Bob :: 0 dot

end note

test->xtokens: transfer()
activate xtokens

note left xtokens
XTokens::transfer(
	Origin::signed(ALICE.into()),
	DOT,
	dot(10f64),
	Box::new(xcm::VersionedMultiLocation::V1(MultiLocation::new(
		1,
		X1(Junction::AccountId32 {
			id: BOB,
			network: NetworkId::Any
		})
	))),
	4_000_000_000
))
end note

xtokens->xtokensimpl: execute_and_send_reserve_kind_xcm()
activate xtokensimpl

note over xtokensimpl
msg = Xcm(vec![
	<b>WithdrawAsset(assets.clone()),
	<b>DepositReserveAsset {
		assets: All.into(),
		max_assets: assets.len() as u32,
		dest: dest.clone(),
		xcm: Xcm(vec![
			Self::buy_execution(fee, &dest, dest_weight)?,
			Self::deposit_asset(recipient, assets.len() as u32),
		]),
	},
]))
end note

xtokensimpl->xcmexec: execute_xcm_in_credit()
activate xcmexec

' WithdrawAsset
xcmexec->vmexec: WithdrawAsset
activate vmexec

vmexec->txcm: withdraw_asset
activate txcm

note over txcm
Event::Burned()
end note

txcm->vmexec:
deactivate txcm

vmexec->xcmexec:
deactivate vmexec

' DepositReserveAsset
xcmexec->vmexec: DepositReserveAsset
activate vmexec

vmexec->xcmSender: send_xcm()
activate xcmSender

xcmSender->vmexec
deactivate xcmSender

vmexec->xcmexec:
deactivate vmexec

xcmexec->xtokensimpl
deactivate xcmexec

note over xtokensimpl
Event::TransferredMultiAssets()
end note

xtokensimpl->xtokens
deactivate xtokensimpl

xtokens->test:
deactivate xtokens

note over test

Alice :: 90 dot
Bob :: 0 dot

end note

@enduml
```
