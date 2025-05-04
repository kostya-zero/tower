"use client";

import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";
import { Dialog, DialogContent } from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { LoaderCircleIcon, RadioTowerIcon } from "lucide-react";
import { useState } from "react";
import { z } from "zod";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";

const connectSchema = z.object({
  serverAddress: z.string().nonempty("Server address is required"),
  username: z.string().nonempty("Username is required"),
});

type ConnectFormValues = z.infer<typeof connectSchema>;

interface ConnectionResult {
  success: boolean;
  message: string;
}

export default function MainPage() {
  const [connecting, setConnecting] = useState(false);

  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<ConnectFormValues>({
    resolver: zodResolver(connectSchema),
    defaultValues: {
      serverAddress: "",
      username: "",
    },
  });

  async function connect(values: ConnectFormValues) {
    setConnecting(true);
    console.log("Form values:", values);
    // await new Promise((resolve) => setTimeout(resolve, 3000));
    invoke("setup_connection", { address: values.serverAddress })
      .then((result) => {
        console.log("Connection result:", result);
      })
      .catch((error) => console.error("Connection error:", error))
      .finally(() => setConnecting(false));
  }

  return (
    <main className="flex min-h-screen items-center justify-between ">
      <div className="m-auto flex flex-col gap-4 max-w-[450px] w-full">
        <div className="flex flex-row gap-4 items-center">
          <RadioTowerIcon className="w-12 h-12 mr-3" />
          <p className="font-funnel font-bold text-6xl">Tower</p>
        </div>
        <p className="font-inter text-sm text-neutral-300">
          Fill server IP-address and your username to connect to a server. Only
          RACv2 servers are compatible.
        </p>
        <form
          className="flex flex-col gap-2"
          onSubmit={handleSubmit(connect)}
          noValidate
        >
          <div className="space-y-1">
            <Input
              disabled={connecting}
              placeholder="Server Address"
              {...register("serverAddress")}
            />
            {errors.serverAddress && (
              <p className="text-xs text-red-500">
                {errors.serverAddress.message}
              </p>
            )}
          </div>
          <div className="space-y-1">
            <Input
              disabled={connecting}
              placeholder="Username"
              {...register("username")}
            />
            {errors.username && (
              <p className="text-xs text-red-500">{errors.username.message}</p>
            )}
          </div>
          <Button
            type="submit"
            disabled={connecting}
            className="cursor-pointer"
          >
            {connecting ? (
              <LoaderCircleIcon className="animate-spin" />
            ) : (
              "Connect"
            )}
          </Button>
        </form>
      </div>
      <Dialog open={connecting}>
        <DialogContent
          onInteractOutside={(e) => e.preventDefault()}
          className="flex flex-row gap-5 p-5 w-[200px] mx-auto items-center"
        >
          <LoaderCircleIcon className="animate-spin" />
          <p>Connecting...</p>
        </DialogContent>
      </Dialog>
    </main>
  );
}
