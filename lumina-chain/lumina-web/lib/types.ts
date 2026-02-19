export type AssetType =
  | { LUSD: null }
  | { LJUN: null }
  | { Lumina: null }
  | { Custom: string };

export type StablecoinInstruction =
  | { FlashMint: { amount: number; collateral_asset: AssetType; collateral_amount: number; commitment: number[] } }
  | { FlashBurn: { amount: number } }
  | { InstantRedeem: { amount: number; destination: number[] } }
  | {
      MintWithCreditScore: {
        amount: number;
        collateral_amount: number;
        credit_score_proof: number[];
        min_score_threshold: number;
        oracle: number[];
      };
    }
  | {
      ListRWA: {
        asset_description: string;
        attested_value: number;
        attestation_proof: number[];
        maturity_date: number | null;
        collateral_eligibility: boolean;
      };
    }
  | { UseRWAAsCollateral: { rwa_id: number; amount_to_pledge: number } }
  | { MintSenior: { amount: number; collateral_amount: number; proof: number[] } }
  | { Transfer: { to: number[]; amount: number; asset: AssetType } };

export type Wallet = {
  publicKey: Uint8Array;
  secretKey: Uint8Array;
};

export type TxReceipt = {
  status: string;
  tx_id: string;
};
