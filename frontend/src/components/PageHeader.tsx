interface PageHeaderProps {
  title: string;
  description?: string;
  children?: React.ReactNode;
  className?: string;
}

export default function PageHeader({ title, description, children, className = "" }: PageHeaderProps) {
  return (
    <div className={`mb-6 sm:mb-8 ${className}`}>
      <h1 className="text-2xl sm:text-3xl font-bold text-gray-900">{title}</h1>
      {description && (
        <p className="mt-2 text-sm sm:text-base text-gray-600">{description}</p>
      )}
      {children}
    </div>
  );
}
