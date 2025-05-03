import type { Metadata } from "next";
import { Funnel_Display, Inter } from "next/font/google";
import "./globals.css";
import { Toaster } from "react-hot-toast";

const inter = Inter({
  subsets: ["latin"],
  variable: "--font-inter",
})

const funnel_display = Funnel_Display({
  subsets: ["latin"],
  variable: "--font-funnel",
})
export const metadata: Metadata = {
  title: "Tower",
  description: "A new description for the Tower project",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body
        className={`antialiased ${inter.variable} ${funnel_display.variable} dark font-inter`}
      >
        {children}
        <Toaster position="top-right" toastOptions={{
          duration: 4000,
          style: {
            background: "#0c0a09",
            color: "#fff"
          }
        }}/>
      </body>
    </html>
  );
}
