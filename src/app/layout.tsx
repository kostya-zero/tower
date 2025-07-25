import type { Metadata } from "next";
import { Funnel_Display, Inter, JetBrains_Mono } from "next/font/google";
import "./globals.css";
import { Toaster } from "@/components/Toasts";

const inter = Inter({
  subsets: ["latin"],
  variable: "--font-inter",
});

const funnel_display = Funnel_Display({
  subsets: ["latin"],
  variable: "--font-funnel",
});

const jetbrains = JetBrains_Mono({
  subsets: ["latin"],
  variable: "--font-jetbrains",
});

export const metadata: Metadata = {
  title: "Tower",
  description: "A RACv2 desktop client.",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body
        className={`antialiased ${inter.variable} ${funnel_display.variable} ${jetbrains.variable} dark font-inter`}
      >
        {children}
        <Toaster />
      </body>
    </html>
  );
}
