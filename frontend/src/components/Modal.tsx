interface ModalProps {
  isOpen: boolean;
  onClose: () => void;
  title: string;
  children: React.ReactNode;
  className?: string;
}

export default function Modal({ isOpen, onClose, title, children, className = "" }: ModalProps) {
  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4">
      {/* Backdrop with blur effect */}
      <div 
        className="absolute inset-0 bg-white bg-opacity-20 backdrop-blur-sm"
        onClick={onClose}
      />
      
      {/* Modal Content */}
      <div 
        className={`relative bg-white rounded-lg shadow-xl w-full max-w-lg border border-gray-200 ${className}`}
        onClick={(e) => e.stopPropagation()}
      >
        {/* Header */}
        <div className="px-6 py-4 border-b border-gray-200">
          <h3 className="text-lg font-semibold text-gray-900">
            {title}
          </h3>
        </div>
        
        {/* Body */}
        <div className="px-6 py-4">
          {children}
        </div>
      </div>
    </div>
  );
}