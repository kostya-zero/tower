import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { RadioTowerIcon } from "lucide-react";

export default function Home() {
  return (
    <main className="flex min-h-screen items-center justify-between ">
      <div className="m-auto flex flex-col gap-4 max-w-[450px] w-full">
        <div className="flex flex-row gap-4 items-center">
          <RadioTowerIcon className="w-12 h-12 mr-3" />
          <p className="font-funnel font-bold text-6xl">Tower</p>
        </div>
        <p className="font-inter">A RACv2 Protocol Client. Made with Tauri.</p>
        <div className="flex flex-col gap-2">
          <Input placeholder="Server Address" />
          <Input placeholder="Username" />
          <Button>Connect</Button>
        </div>
      </div>
    </main>
  );
}
