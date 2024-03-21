use lambdaworks_math::{
    cyclic_group::IsGroup,
    elliptic_curve::{
        short_weierstrass::{
            curves::bls12_381::curve::BLS12381Curve, point::ShortWeierstrassProjectivePoint,
        },
        traits::IsEllipticCurve,
    },
};

fn generate_public_key(private_key: &str) -> ShortWeierstrassProjectivePoint<BLS12381Curve> {
    let priv_key = u64::from_str_radix(private_key, 16).unwrap_or(0);
    let g = BLS12381Curve::generator();
    let public_key = g.operate_with_self(priv_key);
    public_key
}

fn main() {
    let sk = "0x6C616D6264617370";

    let pub_key = generate_public_key(sk);

    println!("{:?}", pub_key);
}
