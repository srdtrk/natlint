// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import { TestInterface } from "TestInterface.sol";

contract TestContract is TestInterface {
    uint256 public value;
    
    event ValueUpdated(uint256 newValue);
    
    error InvalidValue(uint256 value);
    
    enum Status { Active, Inactive, Pending }
    
    struct UserData {
        string name;
        uint256 balance;
    }
    
    type Amount is uint256;

    function setValue(uint256 _value) public {
        if (_value == 0) {
            revert InvalidValue(_value);
        }
        value = _value;
        emit ValueUpdated(_value);
    }
}
