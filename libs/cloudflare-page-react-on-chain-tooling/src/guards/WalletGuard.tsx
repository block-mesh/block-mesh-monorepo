import Faq from '../components/Faq';
import { useWallet } from '@solana/wallet-adapter-react';

import { ComponentType, PropsWithChildren } from "react";
const WalletGuard = ({ children }: PropsWithChildren) => {
    const { connected } = useWallet();
    return (
        <>
            {connected? children : <Faq />}
        </>
    );
};

export const withWalletGuard =
<P extends object>(Component: ComponentType<P>) =>
(props: P) =>
  (
    <WalletGuard>
      <Component {...props} />
    </WalletGuard>
  );

export default withWalletGuard;