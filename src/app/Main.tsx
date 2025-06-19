"use client";

import { invoke } from "@tauri-apps/api/core";
import React, { useState } from "react";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { LoaderCircleIcon, RadioTowerIcon } from "lucide-react";
import { z } from "zod";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { toast } from "@/components/Toasts";
import { AppState } from "@/lib/enums/appstate";
import { version } from "@/lib/constants/version";

const connectSchema = z.object({
  serverAddress: z.string().nonempty("Server address is required"),
  username: z.string().nonempty("Username is required"),
});

type ConnectFormValues = z.infer<typeof connectSchema>;

type Props = {
  setAppState: React.Dispatch<React.SetStateAction<AppState>>;
};

export default function MainPage({ setAppState }: Props) {
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
    invoke("setup_connection", {
      address: values.serverAddress,
      userName: values.username,
    })
      .then(() => {
        setAppState(AppState.Chat);
      })
      .catch((error) =>
        toast({
          title: "Connection Error",
          description: error.toString(),
          button: {
            label: "Retry",
            onClick: () => connect(values),
          },
        }),
      )
      .finally(() => setConnecting(false));
  }

  return (
    <main className="flex min-h-screen items-center justify-between">
      <div className="m-auto flex w-full max-w-[450px] flex-col gap-4">
        <div className="flex flex-row items-center gap-4">
          <RadioTowerIcon className="mr-3 h-12 w-12" />
          <p className="font-funnel text-6xl font-bold">Tower</p>
        </div>
        <p className="font-inter text-sm text-neutral-300">
          Fill server IP-address with port and your username to connect to a
          server. Only RAC v2.x and v1.99.x servers are compatible.
        </p>
        <form
          className="flex flex-col gap-2"
          onSubmit={handleSubmit(connect)}
          noValidate
        >
          <div className="space-y-1">
            <Input
              disabled={connecting}
              placeholder="Server Address and Port"
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
        <small className={"text-neutral-500"}>
          Version {version} • Made by Konstantin Zhigaylo
        </small>
      </div>
      <Dialog open={connecting}>
        <DialogHeader>
          <DialogTitle></DialogTitle>
        </DialogHeader>
        <DialogContent
          onInteractOutside={(e) => e.preventDefault()}
          className="mx-auto flex w-[200px] flex-row items-center gap-5 p-5"
        >
          <LoaderCircleIcon className="animate-spin" />
          <p>Connecting...</p>
        </DialogContent>
      </Dialog>
    </main>
  );
}
