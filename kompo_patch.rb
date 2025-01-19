module Kernel
  alias :org_require :require

  def require path
    Kompo.context do
      org_require path
    end
  rescue LoadError
    org_require path
  end
end
