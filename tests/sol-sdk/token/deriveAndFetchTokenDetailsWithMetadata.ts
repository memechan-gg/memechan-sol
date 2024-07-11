// export async function deriveAndFetchTokenDetailsWithMetadata(
//     programId: string,
//     staticSeed: string,
//     startCounter: number,
//     endCounter: number,
// ): Promise<any[]> {
//     const connection = new Connection('http://localhost:8899', 'confirmed');
//     const programPublicKey = new PublicKey(programId);
//     let tokenDetails = [];

//     for (let counter = startCounter; counter <= endCounter; counter++) {
//         const tokenPDA = findCounterPDA(programPublicKey, counter);

//         try {
//             const token = new Token(connection, tokenPDA, TOKEN_PROGRAM_ID, /* payer */);
//             const tokenInfo = await token.getMintInfo();
//             const metadataPDA = await Metadata.getPDA(tokenPDA);
//             const metadata = await Metadata.load(connection, metadataPDA);

//             tokenDetails.push({
//                 address: tokenPDA.toBase58(),
//                 mintAuthority: tokenInfo.mintAuthority?.toBase58(),
//                 supply: tokenInfo.supply.toString(),
//                 decimals: tokenInfo.decimals,
//                 name: metadata.data.name,
//                 symbol: metadata.data.symbol,
//                 uri: metadata.data.uri,
//                 description: metadata.data.data.description,
//                 twitter: metadata.data.data.twitter,
//                 discord: metadata.data.data.discord
//             });
//         } catch (error) {
//             console.error(`Failed to fetch token details for PDA: ${tokenPDA.toBase58()}`, error);
//         }
//     }

//     return tokenDetails;
// }