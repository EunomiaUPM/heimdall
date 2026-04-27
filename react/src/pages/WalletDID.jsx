import { useState, useEffect } from 'react';
import { Skeleton } from '@/components/ui/skeleton';
import { Button } from '@/components/ui/button';
import { cn } from '@/lib/utils';
import { FileJson, Fingerprint, Key, Link2, Copy, Check, ChevronDown, ChevronUp } from 'lucide-react';

const WalletDID = () => {
  const [didDocument, setDidDocument] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  
  const [isJsonExpanded, setIsJsonExpanded] = useState(false);
  const [copiedId, setCopiedId] = useState(false);

  useEffect(() => {
    const fetchDID = async () => {
      try {
        const response = await fetch('/.well-known/did.json');
        if (!response.ok) {
          throw new Error('Failed to fetch DID document');
        }
        const data = await response.json();
        setDidDocument(data);
        setLoading(false);
      } catch (err) {
        console.error('Error fetching DID:', err);
        setError(err.message);
        setLoading(false);
      }
    };

    fetchDID();
  }, []);

  if (loading) {
    return (
      <div className="space-y-4">
        <h2 className="text-2xl font-bold text-brand-sky mb-4 text-left">DID Document</h2>
        <Skeleton className="h-64 w-full rounded-2xl" />
      </div>
    );
  }

  if (error) {
    return (
      <div className="space-y-4 text-left">
        <h2 className="text-2xl font-bold text-brand-sky mb-4">DID Document</h2>
        <div className="text-danger font-mono text-sm">Error: {error}</div>
      </div>
    );
  }

  const handleCopyId = () => {
    navigator.clipboard.writeText(didDocument?.id || "");
    setCopiedId(true);
    setTimeout(() => setCopiedId(false), 2000);
  };

  return (
    <div className="text-left">
      <h2 className="text-2xl font-bold text-brand-sky mb-6">DID Document</h2>

      <div className="bg-background/50 border border-brand-purple/20 rounded-2xl overflow-hidden shadow-lg shadow-brand-purple/10">
        {/* Header section with DID ID */}
        <div className="p-6 border-b border-brand-purple/20 flex flex-col md:flex-row md:items-start justify-between gap-6">
          <div className="flex items-start gap-4">
            <div className="p-3 rounded-xl bg-brand-purple/10 text-brand-purple shrink-0">
               <Fingerprint className="h-8 w-8" />
            </div>
            <div className="space-y-2">
              <h3 className="font-semibold text-lg flex items-center gap-2 text-foreground">
                Decentralized Identifier
                <span className="inline-flex items-center rounded-full border border-brand-sky/20 bg-brand-sky/10 px-2 py-0.5 text-[10px] font-semibold text-brand-sky uppercase tracking-wide">
                  Active
                </span>
              </h3>
              <div className="flex items-center gap-2 bg-black/40 px-3 py-2 rounded-lg border border-brand-purple/10">
                <span className="font-mono text-sm text-foreground/90 break-all">{didDocument?.id}</span>
                <Button variant="ghost" size="icon" className="h-6 w-6 ml-2 text-muted-foreground hover:text-foreground" onClick={handleCopyId}>
                  {copiedId ? <Check className="h-3 w-3 text-success" /> : <Copy className="h-3 w-3" />}
                </Button>
              </div>
            </div>
          </div>
        </div>

        {/* Content sections */}
        <div className="grid grid-cols-1 md:grid-cols-2 divide-y md:divide-y-0 md:divide-x divide-brand-purple/20">
          
          {/* Verification Methods (Keys) */}
          <div className="p-6 space-y-4">
            <h4 className="text-xs uppercase tracking-widest text-brand-purple/70 font-bold flex items-center gap-2">
              <Key className="h-3.5 w-3.5" />
              Verification Methods
            </h4>
            {didDocument?.verificationMethod && didDocument.verificationMethod.length > 0 ? (
              <div className="space-y-3">
                {didDocument.verificationMethod.map((vm, idx) => (
                  <div key={vm.id || idx} className="bg-black/20 rounded-lg p-3 border border-brand-purple/10 text-xs">
                    <div className="font-mono text-foreground/80 mb-1 truncate">{vm.id}</div>
                    <div className="flex justify-between items-center text-muted-foreground/60 mt-2">
                      <span>Type: {vm.type}</span>
                    </div>
                  </div>
                ))}
              </div>
            ) : (
              <div className="text-sm text-muted-foreground italic">No verification methods defined</div>
            )}
          </div>

          {/* Services */}
          <div className="p-6 space-y-4">
            <h4 className="text-xs uppercase tracking-widest text-brand-purple/70 font-bold flex items-center gap-2">
              <Link2 className="h-3.5 w-3.5" />
              Services
            </h4>
            {didDocument?.service && didDocument.service.length > 0 ? (
              <div className="space-y-3">
                {didDocument.service.map((svc, idx) => (
                  <div key={svc.id || idx} className="bg-black/20 rounded-lg p-3 border border-brand-purple/10 text-xs space-y-1">
                    <div className="font-semibold text-foreground/90 truncate">{svc.type}</div>
                    <div className="font-mono text-foreground/70 truncate break-all opacity-80">{svc.serviceEndpoint}</div>
                  </div>
                ))}
              </div>
            ) : (
              <div className="text-sm text-muted-foreground italic">No services defined</div>
            )}
          </div>
        </div>

        {/* Expandable JSON */}
        <div className="border-t border-brand-purple/20">
          <div 
            onClick={() => setIsJsonExpanded(!isJsonExpanded)}
            className="cursor-pointer p-4 flex items-center justify-between hover:bg-white/5 transition-colors"
          >
            <span className="text-[10px] font-bold uppercase tracking-widest text-brand-purple/70 flex items-center gap-2">
              <FileJson className="h-3 w-3" />
              Raw DID Document
            </span>
            <div className="text-muted-foreground/40">
              {isJsonExpanded ? <ChevronUp className="h-4 w-4" /> : <ChevronDown className="h-4 w-4" />}
            </div>
          </div>
          
          <div className={cn(
            "grid transition-all duration-300 ease-in-out",
            isJsonExpanded ? "grid-rows-[1fr]" : "grid-rows-[0fr]"
          )}>
            <div className="overflow-hidden">
              <div className="p-4 bg-black/40 border-t border-brand-purple/10 shadow-inner">
                <pre className="text-[11px] font-mono text-muted-foreground/80 whitespace-pre-wrap break-all leading-relaxed">
                  {JSON.stringify(didDocument, null, 2)}
                </pre>
              </div>
            </div>
          </div>
        </div>

      </div>
    </div>
  );
};

export default WalletDID;
