import type { Metadata } from "next";
import { JetBrains_Mono } from "next/font/google";
import "./globals.css";
import UserProvider from "./components/UserProvider";

const font = JetBrains_Mono({
  subsets: ["latin"],
  weight: ["100", "200", "300", "400", "500", "600", "700", "800"],
});

export const metadata: Metadata = {
  title: "Lyra",
  description: "Package manager for the Lyra programming language",
};

type Props = {
  children: React.ReactNode;
};

const RootLayout = ({ children }: Props) => (
  <html lang="en">
    <body className={`${font.className} antialiased animated-bg`}>
      <UserProvider>{children}</UserProvider>
    </body>
  </html>
);

export default RootLayout;
