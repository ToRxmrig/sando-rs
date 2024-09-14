// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import "forge-std/Script.sol";
import "forge-std/console.sol";
import "foundry-huff/HuffDeployer.sol";
import "../interfaces/IMetamorphicContractFactory.sol";

contract Deploy is Script {
    IMetamorphicContractFactory factory;
    // serachers
    function setUp() public {
      factory = IMetamorphicContractFactory(0x00000000e82eb0431756271F0d00CFB143685e7B);
    }

    function run() public{
        address sandwich = HuffDeployer.broadcast("sandwich");
        bytes32 salt = bytes32(0xE9D7B0C91a318ED1D016C51b7CC55D9a7A2ADaB562bc4795a02024e288e38303);
        vm.broadcast(0xE9D7B0C91a318ED1D016C51b7CC55D9a7A2ADaB5);
        address metamorphicContract = factory.deployMetamorphicContractFromExistingImplementation(salt, sandwich, "");
        console.log(metamorphicContract);
    }
}
