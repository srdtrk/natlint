// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import { TestInterface } from "TestInterface.sol";

contract TestContract is TestInterface {
    uint256 public value;

    function setValue(uint256 _value) public {
        value = _value;
    }
}
