import { ComponentType, PropsWithChildren } from "react";
import Header from '../components/Header';
import Footer from '../components/Footer';

const MainLayout = ({ children }: PropsWithChildren) => {
  return (
      <>
      <div className="flex flex-col h-screen">
            <header className="flex-shrink-0">
                <Header />
            </header>
            
            <main className="flex-grow overflow-y-auto bg-gray-900 text-white">
                {children}
            </main>
            
            <footer className="flex-shrink-0">
                <Footer />
            </footer>
        </div>
      </>
  );
};

export const withMainLayout =
<P extends object>(Component: ComponentType<P>) =>
(props: P) =>
  (
    <MainLayout>
      <Component {...props} />
    </MainLayout>
  );