use hex_literal::hex;
use node_template_runtime::{
	AccountId, BabeConfig, BalancesConfig, GrandpaConfig, RuntimeGenesisConfig, Signature,
	SudoConfig, SystemConfig, WASM_BINARY, BABE_GENESIS_EPOCH_CONFIG, SessionConfig, StakingConfig, SessionKeys,
	constants::currency::*, StakerStatus, MaxNominations,ImOnlineConfig,
};
use sc_service::ChainType;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sc_telemetry::TelemetryEndpoints;
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill,
};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use node_primitives::*;

// The URL for the telemetry server.
const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<RuntimeGenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

fn session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
	SessionKeys { babe, grandpa, im_online,  authority_discovery}
}

/// Generate an Babe authority key.
pub fn authority_keys_from_seed(s: &str) -> (AccountId, AccountId, BabeId, GrandpaId, ImOnlineId, AuthorityDiscoveryId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", s)),
		get_account_id_from_seed::<sr25519::Public>(s),
		get_from_seed::<BabeId>(s),
		get_from_seed::<GrandpaId>(s),
		get_from_seed::<ImOnlineId>(s),
		get_from_seed::<AuthorityDiscoveryId>(s),
	)
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				vec![],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
				vec![],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		None,
		// Extensions
		None,
	))
}

pub fn staging_network_config() -> ChainSpec {
	let boot_nodes = vec![];

	ChainSpec::from_genesis(
		"Substrate Stencil",
		"stencil_network",
		ChainType::Live,
		staging_network_config_genesis,
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Staging telemetry url is valid; qed"),
		),
		None,
		None,
		None,
		Default::default(),
	)
}

fn staging_network_config_genesis() -> RuntimeGenesisConfig {
	let wasm_binary = WASM_BINARY.expect(
		"Development wasm binary is not available. This means the client is built with \
		 `SKIP_WASM_BUILD` flag and it is only usable for production chains. Please rebuild with \
		 the flag disabled.",
	);

	// for i in 1 2 3 4; do for j in stash controller; do subkey inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in babe; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in grandpa; do subkey --ed25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in im_online; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	let initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId, AuthorityDiscoveryId)> = vec![
		(
			// Stash Account`
			// 5Df94LnKM6ptdaLQVpo9rPbTKBQzEpvhcMUKMbdz4Vqoyj7W
			array_bytes::hex_n_into_unchecked("467f6986687ef250f7d17ee591bf143fe53e1b2b6e541c8409c2498e46bd1d17"),
			// Controller account
			// 5Hjn6chCzYt8zM7ej2PGBxuP1YhQ1GVvHxsiM1c5PP55Cc8p
			array_bytes::hex_n_into_unchecked("fafa03430538ddc9f849f74b42065aba2f0330c7be4b9e879eb87816dac58a15"),
			// Babe Account
			// 5HbD32UJd8JrWDNmBP8zggy8P9HZNypH82LpEYYJ3m2SR5tX
			array_bytes::hex2array_unchecked("f470c0d448a0851086f291ecd432e8e0e924a1d75823626f7b23bf45181a242d")
				.unchecked_into(),
			// Grandpa account
			// 5FD7opppAeP2zDqKgxLj3xMHovd9r81Z8SXzRt3RavxWpQq8
			array_bytes::hex2array_unchecked("8b1f4a2e2bc0953f80e9d6f6be8a41bf2c0d7f22648a7a9bb0876e13769a4477")
				.unchecked_into(),
			// imonline Account
			// 5GQviwyEmDqCgrS98yGSKV7JAmcAAFWmsWZH6JnSz45EeJbC
			array_bytes::hex2array_unchecked("c05d27dd7c4a81338d5f7d9bfdd513f4ead5c09abad10d81460f4ba5a754e12f")
				.unchecked_into(),
			// authority discovery account
			// 5HjeF54rtdV9RHLh9BmHfPSfgW268GiqKU12s2cFYWHUYTof
			array_bytes::hex2array_unchecked("fadf92b35dda22345023fcde913a2de2352445def1a8c436873ab96fe2b02468")
				.unchecked_into(),
		),
		(
			// Stash Account
			// 5DrEnS5Htsp8pKBbc24XVwqqR4PJoJxV3hJ575eWpzA2SMqa
			array_bytes::hex_n_into_unchecked("4ef662d89692be4301db3100d2832a060c9bb24c8003c85c4660dabaaa95630f"),
			// Controller account
			// 5EATfvrz79jWrxXKTc3f6TaQTcCJWR864NZPkyu8DruRxdCd
			array_bytes::hex_n_into_unchecked("5cdc30156a362216f071b3bf23526571ecabf138eb5ca7c9e5c91fcd9b84891d"),
			// Babe Account
			// 5F9jubJxrByyQHr2NtJoBUbUoz1BnPjhfT5AKV8rby5sG3Pm
			array_bytes::hex2array_unchecked("888bd46502e5a1e10a51b3625b0e6ea76d3353dc1bcac6c9da25d586b6ca7e1c")
				.unchecked_into(),
			// Grandpa account
			// 5CV4qVPtFRNG1WHHGo27WU6rjK8qgXRJMsxJhJYxoQjWQzMs
			array_bytes::hex2array_unchecked("1294837c326104a861a447816a286e289786f396d2ded9d5374e40dc812ab91a")
				.unchecked_into(),
			// imonline Account
			// 5DfXEnW4veh4XzbhxxaC6YbVWQWUbDvu5q1SpQuEh3zwfk15
			array_bytes::hex2array_unchecked("46ca13b2862bec637e76fbac53e7f73fb04ae8fbb36e9c4a408614c3b4f93525")
				.unchecked_into(),
			// authority discovery account
			// 5Gdv4kU1T7VoX3AxZtNSdMeKAp6vd1JTDpvw7CJvdsy4UGRE
			array_bytes::hex2array_unchecked("ca4521039b528329801bfe8e83f20414329c17859c1372bb149845f71ebee07c")
				.unchecked_into(),
		),
		(
			// Stash Account
			// 5FWGQMkdhvWHp6Zv9MwxwkTJzyv6EHputu5yaqWfF8QWN8Si
			array_bytes::hex_n_into_unchecked("983365767b9938fdb77847dd099bd60da27577c3860deac79361e2302fda931d"),
			// Controller account
			// 5EZnydpGtW3gdBtY39HNBAq3uPmN3itvaymYZMjd5ZacS1y7
			array_bytes::hex_n_into_unchecked("6ea7d1c9b30e32ebe10ac4142066c23138f08fe951ba6f8328992d57c0e04d15"),
			// Babe Account
			// 5CJi6NASBmh5469yN84kBPgrK89WKYrrDXQGbVTwPckiLuKi
			array_bytes::hex2array_unchecked("0aae3b0a5957982321825e75ddc34aac99237e3a5b0ec1ebc18db3388d7f8a7e")
				.unchecked_into(),
			// Grandpa account
			// 5DEsThhp3DH33H44tdbrpjQQvZiTrsd8xjRKwA3EHXjXyVvY
			array_bytes::hex2array_unchecked("33fd047e281b273e0893d5362b5e62bc680b22e170cd377e96e0b3c75c9a3bcd")
				.unchecked_into(),
			// imonline Account
			// 5Eh8fVQRvXAxzXRUVirqS5vp6gQiiUmDeYJe2SNDoGbBaogd
			array_bytes::hex2array_unchecked("7440cf684dc9100f927d089acd6f9b2543658f924abbfe2c02cd6a3ae03d7f1e")
				.unchecked_into(),
			// authority discovery account
			// 5G3w495Ntwms8WYvNh7w5ZG3sAjUjUwCxgg591mPpMrAFQBE
			array_bytes::hex2array_unchecked("b05a1db8a4bfcf011daf21e22e6b681e88a8084f7dcc420d7f569bfe8e903c68")
				.unchecked_into(),
		),
		(
			// Stash Account
			// 5EZhhh24Roth9at1gVPfSmqXDwAXCKttBcRC7wBTWCrmapZU
			array_bytes::hex_n_into_unchecked("6e9610039163f7042f05e706a13a4a12b02b40531fa4e73b4c42e772f45ca43c"),
			// Controller account
			// 5CD7xuZT763zZcCBFd5zDS2hMus2nQ2psEMqj3ZBvkHvHs6z
			array_bytes::hex_n_into_unchecked("066b1dd329aa2fe7ab347534f5bc7cb2af2035680ee1e8169af78a80cd6db815"),
			// Babe Account
			// 5Gje3aohmTiXdXMSX8ghzFmqf5uciK3eAuvdUq1FebSQAYTb
			array_bytes::hex2array_unchecked("cea2ac41d5baeba039de7bb80b41f0a3022b88f8aa005aab7797f192c0ee4303")
				.unchecked_into(),
			// Grandpa account
			// 5GnBjhSCabfLSzAXyL77MivXRs7bqN4HcCYxJwGQXY6wzXVR
			array_bytes::hex2array_unchecked("d093d8e2cc4358f1714b246aa164b9dd8607bd897f8dee09944df37376a3ae81")
				.unchecked_into(),
			// imonline Account
			// 5FW1voGghSqyBkLBjcNZZR5LHwJJwNnwt1yYP2L2yrWesb3N
			array_bytes::hex2array_unchecked("9802ab26f6de56f9f3c24ed37d6f7c9db634537822a028fd23e1001c81ace515")
				.unchecked_into(),
			// authority discovery account
			// 5HGmzrLGGsTPJPLh7v7mFuXV4otd7aN8LFS8iu8CzbCHtgAW
			array_bytes::hex2array_unchecked("e6620d5f33df7b2b953c65dede20b47ecb571269c2b33ecdab2d7d6645912528")
				.unchecked_into(),
		),
	];

	// generated with secret: subkey inspect "$secret"/fir
	let root_key: AccountId = array_bytes::hex_n_into_unchecked(
		// 5DiKUSBwCfMHrff87PXK7jb5sQHN22PM2QsRgiQfgj1fMngf
		"48ec35b461b474c36b69ea524787bdea45d8cf6cc0e1f38ccc2c2adb5e4ae600",
	);

	let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];

	testnet_genesis(
		wasm_binary,
		initial_authorities,
		vec![],
		root_key,
		endowed_accounts,
		true,
	)
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId,AuthorityDiscoveryId)>,
	initial_nominators: Vec<AccountId>,
	root_key: AccountId,
	mut endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> RuntimeGenesisConfig {
	// endow all authorities and nominators.
	initial_authorities
		.iter()
		.map(|x| &x.0)
		.chain(initial_nominators.iter())
		.for_each(|x| {
			if !endowed_accounts.contains(x) {
				endowed_accounts.push(x.clone())
			}
		});

	// stakers: all validators and nominators.
	const ENDOWMENT: Balance = 10_000_000 * DOLLARS;
	const STASH: Balance = ENDOWMENT / 1000;
	let mut rng = rand::thread_rng();
	let stakers = initial_authorities
		.iter()
		.map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
		.chain(initial_nominators.iter().map(|x| {
			use rand::{seq::SliceRandom, Rng};
			let limit = (MaxNominations::get() as usize).min(initial_authorities.len());
			let count = rng.gen::<usize>() % limit;
			let nominations = initial_authorities
				.as_slice()
				.choose_multiple(&mut rng, count)
				.into_iter()
				.map(|choice| choice.0.clone())
				.collect::<Vec<_>>();
			(x.clone(), x.clone(), STASH, StakerStatus::Nominator(nominations))
		}))
		.collect::<Vec<_>>();
	RuntimeGenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			..Default::default()
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		babe: BabeConfig {
			epoch_config: Some(node_template_runtime::BABE_GENESIS_EPOCH_CONFIG),
			..Default::default()
		},
		grandpa: Default::default(),
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			stakers,
			..Default::default()
		},
		im_online: ImOnlineConfig { keys: vec![] },
		authority_discovery: Default::default(),
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
	}
}
