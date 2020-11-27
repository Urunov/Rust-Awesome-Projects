(function() {var implementors = {};
implementors["core_foundation"] = [{"text":"impl FromStr for CFString","synthetic":false,"types":[]}];
implementors["log"] = [{"text":"impl FromStr for Level","synthetic":false,"types":[]},{"text":"impl FromStr for LevelFilter","synthetic":false,"types":[]}];
implementors["num_rational"] = [{"text":"impl&lt;T:&nbsp;FromStr + Clone + Integer&gt; FromStr for Ratio&lt;T&gt;","synthetic":false,"types":[]}];
implementors["proc_macro2"] = [{"text":"impl FromStr for TokenStream","synthetic":false,"types":[]}];
implementors["shader_version"] = [{"text":"impl FromStr for OpenGL","synthetic":false,"types":[]},{"text":"impl FromStr for GLSL","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()