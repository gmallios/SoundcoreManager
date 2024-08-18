export const BlurredOverlay: React.FC<{ children: React.ReactNode | React.ReactNode[] }> = ({
  children
}) => {
  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-opacity-75 backdrop-blur-sm transition-all">
      {children}
    </div>
  );
};
