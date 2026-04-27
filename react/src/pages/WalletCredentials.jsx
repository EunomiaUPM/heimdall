import { useState, useEffect, useCallback } from 'react';
import { VITE_API_SERVER_URL as apiUrl } from '@/lib/api';
import { Button } from '@/components/ui/button';
import { Skeleton } from '@/components/ui/skeleton';
import { cn } from '@/lib/utils';
import { 
  ShieldAlert, ShieldCheck, ChevronDown, ChevronUp, FileJson, 
  Fingerprint, Calendar, Building2, Loader2, Info, Trash2, Check 
} from 'lucide-react';

const COMPLIANCE_TYPES = [
  "gx:Eori", "gx:Euid", "gx:LeiCode", "gx:LocalRegistrationNumber", "gx:TaxId", "gx:VatId",
  "Eori", "TaxId", "VatId"
];

const CredentialCard = ({ vc, onRefetch }) => {
  const [isExpanded, setIsExpanded] = useState(false);
  const [isDeleting, setIsDeleting] = useState(false);
  const doc = vc.parsedDocument || {};

  const type = doc.type ? (Array.isArray(doc.type) ? doc.type[doc.type.length - 1] : doc.type) : "VerifiableCredential";
  
  let issuerName = "Unknown Issuer";
  if (doc.issuer) {
    if (typeof doc.issuer === 'string') {
      issuerName = doc.issuer.split(':').pop() || doc.issuer;
    } else if (doc.issuer.name) {
      issuerName = doc.issuer.name;
    } else if (doc.issuer.id) {
      issuerName = doc.issuer.id.split(':').pop() || doc.issuer.id;
    }
  }

  const handleDelete = async (e) => {
    e.stopPropagation();
    setIsDeleting(true);
    try {
      const response = await fetch(`${apiUrl}/wallet/credential/${vc.id}`, { method: 'DELETE' });
      if (!response.ok) throw new Error("Failed to delete credential");
      onRefetch();
    } catch (err) {
      console.error("Error deleting VC:", err);
      setIsDeleting(false);
    }
  };

  return (
    <div className="group border border-brand-purple/20 rounded-xl overflow-hidden bg-background/50 transition-all hover:bg-background/70 shadow-md">
      <div className="p-5 flex flex-col md:flex-row md:items-center justify-between gap-4">
        <div className="flex items-start gap-4">
          <div className="p-3 rounded-xl bg-brand-purple/10 text-brand-purple shrink-0">
             <Fingerprint className="h-6 w-6" />
          </div>
          <div className="space-y-1.5">
            <h3 className="font-semibold text-lg flex items-center gap-2">
              {type.replace(/([A-Z])/g, ' $1').trim()}
              <span className="inline-flex items-center rounded-full border border-brand-sky/20 bg-brand-sky/10 px-2 py-0.5 text-[10px] font-semibold text-brand-sky">
                Active
              </span>
            </h3>
            <div className="flex flex-wrap items-center gap-x-4 gap-y-2 text-sm text-muted-foreground">
              <span className="flex items-center gap-1">
                <Building2 className="h-3.5 w-3.5" />
                {issuerName}
              </span>
              {doc.issuanceDate && (
                <span className="flex items-center gap-1">
                  <Calendar className="h-3.5 w-3.5" />
                  {new Date(doc.issuanceDate).toLocaleDateString()}
                </span>
              )}
            </div>
          </div>
        </div>
        
        <Button 
          variant="outline" 
          onClick={() => setIsExpanded(!isExpanded)}
          className="shrink-0 border-brand-purple/20 hover:bg-brand-purple/10 text-brand-purple"
        >
          {isExpanded ? (
            <><ChevronUp className="h-4 w-4 mr-2" /> Hide Details</>
          ) : (
            <><ChevronDown className="h-4 w-4 mr-2" /> View Details</>
          )}
        </Button>
      </div>

      <div className={cn(
        "grid transition-all duration-300 ease-in-out",
        isExpanded ? "grid-rows-[1fr]" : "grid-rows-[0fr]"
      )}>
        <div className="overflow-hidden">
          <div className="p-4 bg-black/30 border-t border-brand-purple/20">
            <div className="flex items-center justify-between mb-3 px-2">
               <span className="text-[10px] font-bold uppercase tracking-widest text-brand-purple/70 flex items-center gap-2">
                 <FileJson className="h-3 w-3" />
                 Raw Document
               </span>
               <Button 
                 variant="ghost" 
                 size="sm" 
                 onClick={handleDelete} 
                 disabled={isDeleting}
                 className="h-7 text-xs text-danger hover:text-danger hover:bg-danger/10"
               >
                 {isDeleting ? <Loader2 className="h-3 w-3 animate-spin mr-1.5" /> : <Trash2 className="h-3 w-3 mr-1.5" />}
                 Delete Credential
               </Button>
            </div>
            <pre className="text-xs font-mono text-muted-foreground/80 whitespace-pre-wrap break-all leading-relaxed bg-black/40 p-4 rounded border border-brand-purple/10">
              {JSON.stringify(vc, null, 2)}
            </pre>
          </div>
        </div>
      </div>
    </div>
  );
};

const WalletCredentials = () => {
  const [credentials, setCredentials] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  
  const [isGenerating, setIsGenerating] = useState(false);
  const [generateSuccess, setGenerateSuccess] = useState(false);

  const fetchCredentials = useCallback(async () => {
    try {
      const response = await fetch(`${apiUrl}/wallet/vcs`);
      if (!response.ok) throw new Error('Failed to fetch credentials');
      const data = await response.json();
      setCredentials(Array.isArray(data) ? data : [data]);
      setLoading(false);
    } catch (err) {
      console.error('Error fetching credentials:', err);
      setError(err.message);
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchCredentials();
  }, [fetchCredentials]);

  const hasRegistrationVC = credentials.some(vc => {
    const types = vc.parsedDocument?.type || [];
    return types.some(t => COMPLIANCE_TYPES.includes(t));
  });

  const hasGaiaLabel = credentials.some(vc => {
    const types = vc.parsedDocument?.type || [];
    return types.some(t => t === "gx:LabelCredential" || t === "LabelCredential");
  });

  const handleGenerateGaia = async () => {
    setIsGenerating(true);
    setGenerateSuccess(false);
    try {
      const response = await fetch(`${apiUrl}/gaia/credential/generate`, { method: "POST" });
      if (!response.ok) throw new Error("Failed to generate Gaia-X credential");
      setGenerateSuccess(true);
      await fetchCredentials();
    } catch (err) {
      console.error("Error generating Gaia-X credentials:", err);
    } finally {
      setIsGenerating(false);
    }
  };

  if (loading) {
    return (
      <div className="space-y-4">
        <Skeleton className="h-48 w-full rounded-2xl" />
        <Skeleton className="h-32 w-full rounded-xl" />
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex flex-col items-center justify-center py-20 text-danger font-mono text-xs">
        <ShieldAlert className="h-8 w-8 mb-2" />
        Error loading credentials
      </div>
    );
  }

  return (
    <div className="space-y-8 pb-20 text-left">
      {/* Gaia-X Compliance Section */}
      <div>
        <h2 className="text-2xl font-bold text-brand-sky mb-4">Gaia-X Compliance</h2>
        <div className={cn(
          "relative overflow-hidden p-6 rounded-2xl border transition-all duration-300",
          hasGaiaLabel ? "bg-success/10 border-success/20 shadow-lg" : 
          hasRegistrationVC 
            ? "bg-brand-purple/10 border-brand-purple/30 shadow-lg shadow-brand-purple/10" 
            : "bg-background/50 border-brand-purple/10 opacity-80"
        )}>
          <div className="flex flex-col md:flex-row md:items-center justify-between gap-6">
            <div className="space-y-2">
              <div className="flex items-center gap-2">
                <ShieldCheck className={cn("h-5 w-5", hasGaiaLabel ? "text-success" : hasRegistrationVC ? "text-brand-purple" : "text-muted-foreground")} />
                <h3 className="font-semibold text-lg text-foreground">Gaia-X Framework Setup</h3>
              </div>
              <p className="text-sm text-muted-foreground max-w-xl">
                {hasGaiaLabel ? 
                  "Your wallet is already Gaia-X compliant. You possess a valid LabelCredential." :
                  "Generate the necessary Gaia-X compliant credentials based on your registration information. This will enable your participation in Gaia-X ecosystems."
                }
              </p>
              {generateSuccess && !hasGaiaLabel && (
                <div className="flex items-center gap-2 text-xs font-medium text-success animate-in fade-in slide-in-from-left-2 mt-2">
                   <Info className="h-3 w-3" />
                   Gaia-X credentials generated successfully.
                </div>
              )}
            </div>

            <Button 
              disabled={hasGaiaLabel || !hasRegistrationVC || isGenerating}
              onClick={handleGenerateGaia}
              className={cn("md:min-w-[200px] font-bold text-white", hasGaiaLabel ? "" : "bg-brand-purple hover:bg-brand-purple/80")}
              variant={hasGaiaLabel ? "outline" : "default"}
            >
              {isGenerating ? (
                <><Loader2 className="mr-2 h-4 w-4 animate-spin" /> Generating...</>
              ) : hasGaiaLabel ? (
                <><Check className="mr-2 h-4 w-4 text-success" /> Compliance Active</>
              ) : (
                "Setup Gaia-X Compliance"
              )}
            </Button>
          </div>
          
          {!hasRegistrationVC && !hasGaiaLabel && (
            <div className="mt-4 p-3 rounded-lg bg-warning/10 border border-warning/20 text-[10px] text-warning flex items-center gap-2 font-mono">
              <ShieldAlert className="h-3 w-3" />
              REQUIREMENT: You need an EORI, TaxID, or VAT registration credential to enable this action.
            </div>
          )}
        </div>
      </div>

      {/* Credentials List */}
      <div>
        <h2 className="text-2xl font-bold text-brand-sky mb-4">Verifiable Credentials</h2>
        {credentials.length === 0 ? (
          <div className="rounded-xl border border-brand-purple/20 bg-background/50 p-12 text-center shadow-lg flex flex-col items-center justify-center">
             <FileJson className="h-10 w-10 text-muted-foreground/30 mb-4" />
             <p className="text-muted-foreground text-lg font-medium">No credentials found</p>
             <p className="text-sm text-muted-foreground/60 mt-1">Credentials will appear here once issued to your wallet.</p>
          </div>
        ) : (
          <div className="flex flex-col gap-4">
            {credentials.map((vc, idx) => (
              <CredentialCard key={vc.id || idx} vc={vc} onRefetch={fetchCredentials} />
            ))}
          </div>
        )}
      </div>
    </div>
  );
};

export default WalletCredentials;
