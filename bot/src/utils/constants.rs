use std::str::FromStr;

use ethers::prelude::*;
use indoc::indoc;

// Return weth address
pub fn get_weth_address() -> Address {
    Address::from_str("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2").unwrap()
}

// Return uniswap v3 quoter address
pub fn get_uniswap_v3_quoter_address() -> Address {
    Address::from_str("0xb27308f9F90D607463bb33eA1BeBb41C27CE5AB6").unwrap()
}

pub fn get_sqrt_price_limit_x96(is_zero_for_one: bool) -> U256 {
    if is_zero_for_one {
        U256::from_str("0x1000276AD").unwrap()
    } else {
        U256::from_str("0xFFFD8963EFD1FC6A506488495D951D5263988D25").unwrap()
    }
}

// Return the ethdev address (used if we need funds)
pub fn get_eth_dev() -> Address {
    Address::from_str("0x5AbFEc25f74Cd88437631a7731906932776356f9").unwrap()
}

// Returns the bytecode for our custom modded router contract
pub fn get_braindance_code() -> Bytes {
    "608060405234801561001057600080fd5b506004361061004c5760003560e01c80634b588d401461005157806381eeb93c1461007d57806390063e5914610090578063fa461e33146100a5575b600080fd5b61006461005f366004610994565b6100b8565b6040805192835260208301919091520160405180910390f35b61006461008b366004610994565b61023d565b6100a361009e3660046109e7565b610502565b005b6100a36100b3366004610a46565b61061d565b600080846001600160a01b038085169086161082816100eb5773fffd8963efd1fc6a506488495d951d5263988d256100f2565b6401000276ad5b90506000828860405160200161011d92919091151582526001600160a01b0316602082015260400190565b6040516020818303038152906040529050600080856001600160a01b031663128acb0830878f88886040518663ffffffff1660e01b8152600401610165959493929190610b13565b60408051808303816000875af1158015610183573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906101a79190610b4e565b91509150846101b657816101b8565b805b6101c190610b88565b6040516370a0823160e01b81523060048201529098506001600160a01b038a16906370a0823190602401602060405180830381865afa158015610208573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061022c9190610ba4565b965050505050505094509492505050565b60405163a9059cbb60e01b81526001600160a01b03848116600483015260248201869052600091829185169063a9059cbb906044016020604051808303816000875af1158015610291573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906102b59190610bcb565b50600080600080886001600160a01b0316630902f1ac6040518163ffffffff1660e01b8152600401606060405180830381865afa1580156102fa573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061031e9190610c0b565b506001600160701b031691506001600160701b03169150866001600160a01b0316886001600160a01b0316101561035a57819350809250610361565b8093508192505b50506040516370a0823160e01b81526001600160a01b0388811660048301526000916103dd918591908a16906370a0823190602401602060405180830381865afa1580156103b3573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103d79190610ba4565b90610740565b90506103ea8184846107a1565b9450600080876001600160a01b0316896001600160a01b03161061041057866000610414565b6000875b6040805160008152602081019182905263022c0d9f60e01b90915291935091506001600160a01b038b169063022c0d9f906104589085908590309060248101610c5b565b600060405180830381600087803b15801561047257600080fd5b505af1158015610486573d6000803e3d6000fd5b50506040516370a0823160e01b81523060048201526001600160a01b038b1692506370a082319150602401602060405180830381865afa1580156104ce573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906104f29190610ba4565b9550505050505094509492505050565b60405163a9059cbb60e01b81526001600160a01b0384811660048301526024820187905283169063a9059cbb906044016020604051808303816000875af1158015610551573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105759190610bcb565b50600080826001600160a01b0316846001600160a01b03161061059a5785600061059e565b6000865b6040805160008152602081019182905263022c0d9f60e01b90915291935091506001600160a01b0386169063022c0d9f906105e29085908590309060248101610c5b565b600060405180830381600087803b1580156105fc57600080fd5b505af1158015610610573d6000803e3d6000fd5b5050505050505050505050565b600084138061062c5750600083135b61063557600080fd5b60008061064483850185610c92565b9150915081156106c55760405163a9059cbb60e01b8152336004820152602481018790526001600160a01b0382169063a9059cbb906044016020604051808303816000875af115801561069b573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106bf9190610bcb565b50610738565b60405163a9059cbb60e01b8152336004820152602481018690526001600160a01b0382169063a9059cbb906044016020604051808303816000875af1158015610712573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107369190610bcb565b505b505050505050565b60008261074d8382610ccb565b915081111561079b5760405162461bcd60e51b815260206004820152601560248201527464732d6d6174682d7375622d756e646572666c6f7760581b60448201526064015b60405180910390fd5b92915050565b60008084116108065760405162461bcd60e51b815260206004820152602b60248201527f556e697377617056324c6962726172793a20494e53554646494349454e545f4960448201526a1394155517d05353d5539560aa1b6064820152608401610792565b6000831180156108165750600082115b6108735760405162461bcd60e51b815260206004820152602860248201527f556e697377617056324c6962726172793a20494e53554646494349454e545f4c604482015267495155494449545960c01b6064820152608401610792565b6000610881856103e56108c0565b9050600061088f82856108c0565b905060006108a9836108a3886103e86108c0565b90610927565b90506108b58183610ce2565b979650505050505050565b60008115806108e4575082826108d68183610d04565b92506108e29083610ce2565b145b61079b5760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6d756c2d6f766572666c6f7760601b6044820152606401610792565b6000826109348382610d23565b915081101561079b5760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6164642d6f766572666c6f7760601b6044820152606401610792565b6001600160a01b038116811461099157600080fd5b50565b600080600080608085870312156109aa57600080fd5b8435935060208501356109bc8161097c565b925060408501356109cc8161097c565b915060608501356109dc8161097c565b939692955090935050565b600080600080600060a086880312156109ff57600080fd5b85359450602086013593506040860135610a188161097c565b92506060860135610a288161097c565b91506080860135610a388161097c565b809150509295509295909350565b60008060008060608587031215610a5c57600080fd5b8435935060208501359250604085013567ffffffffffffffff80821115610a8257600080fd5b818701915087601f830112610a9657600080fd5b813581811115610aa557600080fd5b886020828501011115610ab757600080fd5b95989497505060200194505050565b6000815180845260005b81811015610aec57602081850181015186830182015201610ad0565b81811115610afe576000602083870101525b50601f01601f19169290920160200192915050565b6001600160a01b0386811682528515156020830152604082018590528316606082015260a0608082018190526000906108b590830184610ac6565b60008060408385031215610b6157600080fd5b505080516020909101519092909150565b634e487b7160e01b600052601160045260246000fd5b6000600160ff1b8201610b9d57610b9d610b72565b5060000390565b600060208284031215610bb657600080fd5b5051919050565b801515811461099157600080fd5b600060208284031215610bdd57600080fd5b8151610be881610bbd565b9392505050565b80516001600160701b0381168114610c0657600080fd5b919050565b600080600060608486031215610c2057600080fd5b610c2984610bef565b9250610c3760208501610bef565b9150604084015163ffffffff81168114610c5057600080fd5b809150509250925092565b84815283602082015260018060a01b0383166040820152608060608201526000610c886080830184610ac6565b9695505050505050565b60008060408385031215610ca557600080fd5b8235610cb081610bbd565b91506020830135610cc08161097c565b809150509250929050565b600082821015610cdd57610cdd610b72565b500390565b600082610cff57634e487b7160e01b600052601260045260246000fd5b500490565b6000816000190483118215151615610d1e57610d1e610b72565b500290565b60008219821115610d3657610d36610b72565b50019056fea2646970667358221220acb668db58d51617c0d50e902950ba737188460329e57df8dc4a043d4483bdad64736f6c634300080f0033".parse().unwrap()
}

// Return runtime code for our sandwich contract (if u want to test new contract impl)
pub fn get_test_sandwich_code() -> Bytes {
    "730f91479f971bd0b98629311b6c9052b8363bc9a5331461001f57610086565b3d353d1a565b005b6101da565b61027a565b610305565b61067e565b61074e565b610807565b6108a7565b61038e565b610455565b610530565b6105c5565b610d4c565b610968565b610a71565b610b67565b610c50565b610d53565b610d57565b610dab565b730f91479f971bd0b98629311b6c9052b8363bc9a5321415610df8576099357fff000000000000000000000000000000000000000000000000000000000000003d527f1f98431c8ad98523631ae4a59f267346ea31f98400000000000000000000000046526015527fe34f199b19b2b4f47f68442619d555527d244f78a3297ea89325f843f87b8b5460355260553d2073ffffffffffffffffffffffffffffffffffffffff16331415610025573d3d60443d3d7effffffffffffffffffffffffffffffffffffffff00000000000000000000006084351660581c60843560f81c6101a2577fa9059cbb000000000000000000000000000000000000000000000000000000003d52336004526024356024525af161002557610df8565b7fa9059cbb000000000000000000000000000000000000000000000000000000003d52336004526004356024525af161002557610df8565b3d3d60a43d3d60023560601c7f23b872dd000000000000000000000000000000000000000000000000000000003d523060045234604052806024523d3d60643d3d73c02aaa39b223fe8d0a0e5c4f27ead9083c756cc25af1507f022c0d9f000000000000000000000000000000000000000000000000000000005f525f6004525f60245260163546355f1a523060445260806064525af161002557610df4565b3d3d60a43d3d60023560601c7fa9059cbb000000000000000000000000000000000000000000000000000000003d5280600452602a3546353d1a523d3d60443d3d60163560601c5af1505f6004525f6024527f022c0d9f0000000000000000000000000000000000000000000000000000000034015f523060445260806064525af161002557610df4565b3d3d60a43d3d60023560601c7fa9059cbb000000000000000000000000000000000000000000000000000000003d5280600452602a3546353d1a523d3d60443d3d60163560601c5af1507f022c0d9f000000000000000000000000000000000000000000000000000000005f525f600452346020523060445260806064525af161002557610df4565b3d3d60a43d3d60023560601c6016357fffffffff000000000000000000000000000000000000000000000000000000001646353d3d3d7f022c0d9f000000000000000000000000000000000000000000000000000000003d7f23b872dd000000000000000000000000000000000000000000000000000000003d523060045234604052876024523d3d60643d3d73c02aaa39b223fe8d0a0e5c4f27ead9083c756cc25af150526004526024521a523060445260806064525af1601a90601935461a57610df4565b3d3d60a43d3d856002013560601c7f23b872dd000000000000000000000000000000000000000000000000000000003d52306004523d60445286601a013560d81c604052806024523d3d60643d3d73c02aaa39b223fe8d0a0e5c4f27ead9083c756cc25af1507f022c0d9f000000000000000000000000000000000000000000000000000000005f525f6004525f60245286601601357fffffffff00000000000000000000000000000000000000000000000000000000168735461a523060445260806064525f6084525af190601f019081355f1a57610df8565b3d3d60a460403d60023560601c7fa9059cbb000000000000000000000000000000000000000000000000000000003d52602a357fffffffff000000000000000000000000000000000000000000000000000000001646353d1a52806004523d3d60443d3d60163560601c5af15034602d35461a5263022c0d9f60245230608452608060a4525af1602f90602e35461a57610df4565b466024525f8060a4604082856002013560601c7fa9059cbb00000000000000000000000000000000000000000000000000000000825286602a01357fffffffff00000000000000000000000000000000000000000000000000000000168735461a52806004525f80604481808b6016013560601c5af1505f6044525f606452866013013564ffffffffff168760320135461a5263022c0d9f60245230608452608060a4525f60c4525af1906034019081355f1a57610df8565b7f128acb08000000000000000000000000000000000000000000000000000000005f5230600452466024526016357fffffffffff000000000000000000000000000000000000000000000000000000166015353d1a1c3d036044526401000276ad60645260a0608452603560a4527f010000000000000000000000000000000000000000000000000000000000000073c02aaa39b223fe8d0a0e5c4f27ead9083c756cc260581b0160c452601b3560d95260403d60f93d3d463560601c5af15f513460201b111661002557610df4565b7f128acb08000000000000000000000000000000000000000000000000000000003d52306004523460201b60445273fffd8963efd1fc6a506488495d951d5263988d2560645260a0608452603560a45273c02aaa39b223fe8d0a0e5c4f27ead9083c756cc260581b60c452601b3560d95260403d60f93d3d463560601c5af13d513d036016357fffffffffff000000000000000000000000000000000000000000000000000000166015353d1a1c101661002557610df4565b7f128acb08000000000000000000000000000000000000000000000000000000003d52306004526016357fffffffffff000000000000000000000000000000000000000000000000000000166015353d1a1c60445273fffd8963efd1fc6a506488495d951d5263988d2560645260a0608452603560a452601b3560601c60581b60c452602f3560d95260403d60f93d3d463560601c5af161002557610df4565b7f128acb08000000000000000000000000000000000000000000000000000000003d5230600452466024523460201b3d036044526401000276ad60645260a0608452603560a4527f0100000000000000000000000000000000000000000000000000000000000000601b3560601c60581b0160c452602f3560d95260403d60f93d3d463560601c5af13d516016357fffffffffff000000000000000000000000000000000000000000000000000000166015353d1a1c111661002557610df4565b7f128acb08000000000000000000000000000000000000000000000000000000005f52306004524660245280601b01357fffffffffff0000000000000000000000000000000000000000000000000000001681601a01355f1a1c5f036044526401000276ad60645260a0608452603560a4527f010000000000000000000000000000000000000000000000000000000000000073c02aaa39b223fe8d0a0e5c4f27ead9083c756cc260581b0160c452806020013560d95260405f60f95f5f8546013560601c5af15f5182601601357fffffffff000000000000000000000000000000000000000000000000000000001683601501355f1a1c1116906040019081355f1a57610df8565b7f128acb08000000000000000000000000000000000000000000000000000000005f52306004525f60245280601601357fffffffff000000000000000000000000000000000000000000000000000000001681601501355f1a1c60445273fffd8963efd1fc6a506488495d951d5263988d2560645260a0608452603560a45273c02aaa39b223fe8d0a0e5c4f27ead9083c756cc260581b60c452806020013560d95260405f60f95f5f8546013560601c5af15f513d0382601b01357fffffffffff0000000000000000000000000000000000000000000000000000001683601a01355f1a1c1016906040019081355f1a57610df8565b7f128acb08000000000000000000000000000000000000000000000000000000005f52306004525f60245280601b01357fffffffffff0000000000000000000000000000000000000000000000000000001681601a01355f1a1c60445273fffd8963efd1fc6a506488495d951d5263988d2560645260a0608452603560a452806020013560601c60581b60c452806034013560d95260405f60f95f5f8546013560601c5af15f513d0382601601357fffffffff000000000000000000000000000000000000000000000000000000001683601501355f1a1c1016906054019081355f1a57610df8565b7f128acb08000000000000000000000000000000000000000000000000000000005f52306004524660245280601601357fffffffff000000000000000000000000000000000000000000000000000000001681601501355f1a1c5f036044526401000276ad60645260a0608452603560a4527f0100000000000000000000000000000000000000000000000000000000000000816020013560601c60581b0160c452806034013560d95260405f60f95f5f8546013560601c5af15f5182601b01357fffffffffff0000000000000000000000000000000000000000000000000000001683601a01355f1a1c1116906054019081355f1a57610df8565b463d35461a565b33ff005b7f2e1a7d4d0000000000000000000000000000000000000000000000000000000034013d523d3d60243d3d73c02aaa39b223fe8d0a0e5c4f27ead9083c756cc25af13d3d3d3d47335af11661002557610df4565b7fd0e30db0000000000000000000000000000000000000000000000000000000003d523d3d60043d3473c02aaa39b223fe8d0a0e5c4f27ead9083c756cc25af161002557610df4565b5f80fd5b4680fd".parse().unwrap()
}

// Return the event signature to a erc20 transfer
pub fn get_erc20_transfer_event_signature() -> H256 {
    H256::from_str("0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef").unwrap()
}

pub fn get_end_of_multi_payload() -> u8 {
    37_u8
}

pub fn get_prepare_stack_payload() -> u8 {
    94_u8
}

pub fn get_banner() -> &'static str {
    let banner = indoc! {
r#"

 _____   ___   _   _ ______  _____         ______  _____
/  ___| / _ \ | \ | ||  _  \|  _  |        | ___ \/  ___|
\ `--. / /_\ \|  \| || | | || | | | ______ | |_/ /\ `--.
 `--. \|  _  || . ` || | | || | | ||______||    /  `--. \
/\__/ /| | | || |\  || |/ / \ \_/ /        | |\ \ /\__/ /
\____/ \_| |_/\_| \_/|___/   \___/         \_| \_|\____/

"#};
    banner
}
