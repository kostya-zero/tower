import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";

type Props = {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onConfirm: () => void;
};

export default function DisconnectDialog({
  open,
  onOpenChange,
  onConfirm,
}: Props) {
  const handleConfirm = () => {
    onOpenChange(false);
    onConfirm();
  };
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Disconnect from server</DialogTitle>
          <DialogDescription>
            Are you sure you want to disconnect from the server?
          </DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <DialogClose asChild>
            <Button variant={"outline"}>Cancel</Button>
          </DialogClose>
          <Button onClick={handleConfirm}>Disconnect</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
