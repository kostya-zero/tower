type Props = {
  rawString: string;
};

export default function BadMessageSection({ rawString }: Props) {
  return (
    <figure className="flex flex-col p-4 gap-1 border-b border-neutral-800">
      <p className="text-red-400 text-sm">Failed to parse this message:</p>
      <p className="font-jetbrains tracking-tight">{rawString}</p>
      <p className="text-xs text-neutral-500">
        This block is shown because Tower failed to parse this message. Usually
        it happens when client that sent this message is unknown to Tower.
      </p>
    </figure>
  );
}
