import { toast as sonnerToast, Toaster as Sonner } from "sonner";
import { Button } from "@/components/ui/button";
import { X } from "lucide-react";

const Toaster = () => {
  return <Sonner position={"top-right"} />;
};

interface BottomButton {
  label: string;
  onClick: () => void;
  primary?: boolean;
}

interface ToastProps {
  id: string | number;
  title: string;
  description?: string;
  bottomButtons?: Array<BottomButton>;
  button?: {
    label: string;
    onClick: () => void;
  };
}

const Toast = (props: ToastProps) => {
  const { title, description, button, id } = props;
  return (
    <div className="group flex w-full items-end gap-4 rounded-lg border border-neutral-800 bg-neutral-900 p-4 md:w-[364px] md:max-w-[364px]">
      <div className="flex flex-1 items-center">
        <div className="w-full text-sm">
          <div className={"flex flex-row items-center justify-between"}>
            <p className="font-semibold text-white">{title}</p>
            {!button && (
              <X
                onClick={() => sonnerToast.dismiss(id)}
                size={16}
                className={
                  "0 absolute right-0 mr-4 text-neutral-100/40 opacity-0 transition duration-200 group-hover:opacity-100 hover:text-neutral-50"
                }
              />
            )}
          </div>
          {description && (
            <p className="mt-1 text-neutral-300">{description}</p>
          )}
        </div>
      </div>
      {button && (
        <Button
          onClick={() => {
            button.onClick();
            sonnerToast.dismiss(id);
          }}
          variant={"outline"}
        >
          {button.label}
        </Button>
      )}
    </div>
  );
};

function toast(toast: Omit<ToastProps, "id">) {
  return sonnerToast.custom((id) => (
    <Toast
      id={id}
      title={toast.title}
      description={toast.description}
      button={toast.button}
    />
  ));
}

export { Toaster, toast };
