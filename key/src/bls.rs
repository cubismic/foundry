// Copyright 2018 Kodebox, Inc.
// This file is part of CodeChain.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use bls_sigs_ref::{BLSSigCore, BLSSignatureBasic};
use pairing_plus::bls12_381::{G1, G2};
use pairing_plus::bls12_381::fr::Fr;

use std::fmt;

impl Eq for BLSSignature {}

impl fmt::Debug for BLSSignatureBasic {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("Signature").field
    }
}

impl fmt::Display for BLSSignatureBasic {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_hex())
    }
}

/*impl Default for BLSSignatureBasic {
    fn default() -> self {
        BLSSignature([0; 48])
    }
}*/

/*impl Hash for BLSSignature {
    fn hash<H: Hasher>(&self, state: &mut H) {
        H512::from(self.0).hash(state)
    }
}*/
/*
impl Clone for BLSSignature {
    fn clone(&self) -> Self {
        BLSSignature(self.0)
    }
}

impl From<[u8; 48]> for BLSSignature {
    fn from(s: [u8; 48]) -> Self {
        BLSSignature(s)
    }
}

impl From<BLSSignature> for [u8; 48] {
    fn from(s: BLSSignature) -> Self {
        s.0
    }
}

impl<'a> From<&'a [u8]> for BLSSignature {
    fn from(s: [u8; 48]) -> Self {
        BLSSignature(s)
    }
}
*/
/*impl From<BLSSignature> for H256 {
    fn from(s: BLSSignature) -> Self {
        H256::from(s.0)
    }
}*/

/*impl From<H256> for BLSSignature {
    fn from(bytes: H256) -> Self {
        BLSSignature(bytes.into())
    }
}*/

/*
impl Deref for BLSSignature {
    type Target = [u8; 48];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BLSSignature {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}*/

/*impl Serialize for BLSSignature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer, {
        let data: H256 = self.0.into();
        data.serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for BLSSignature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'a>, {
        let data = H256::deserialize(deserializer)?;
        Ok(Self::from(data))
    }
}

impl Encodable for BLSSignature {
    fn rlp_append(&self, s: &mut RlpStream) {
        let data: H256 = self.0.into();
        data.rlp_append(s);
    }
}

impl Decodable for BLSSignature {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        let data = H256::decode(rlp)?;
        Ok(BLSSignature::from(data))
    }
}*/



pub fn sign_bls(private: Fr, message: AsRef<[u8]>) ->  {
    /// TODO
    let sig = G1::sign(private, message);
    Ok(sig)
}

pub fn verify_bls(public: &BLSPublic, signature: &BLSSignature, message: &Message) -> Result<bool, Error> {
    /// TODO
    match G1::verify(public, signature, message) {
        true => Ok(true)
        false => Ok(false)
    }
}

pub fn verify_bls_address(address: &Address, signature: &BLSSignature, message: &Message) -> Result<bool, Error> {
    let public = recover_bls(signature, message)
    let recovered_address = public_to_address(&public);
    Ok(address == &recovered_address)
}
