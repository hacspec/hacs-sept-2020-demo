(function() {var implementors = {};
implementors["num_bigint"] = [{"text":"impl Hash for Sign","synthetic":false,"types":[]},{"text":"impl Hash for BigInt","synthetic":false,"types":[]},{"text":"impl Hash for BigUint","synthetic":false,"types":[]}];
implementors["num_complex"] = [{"text":"impl&lt;T:&nbsp;Hash&gt; Hash for Complex&lt;T&gt;","synthetic":false,"types":[]}];
implementors["num_rational"] = [{"text":"impl&lt;T:&nbsp;Clone + Integer + Hash&gt; Hash for Ratio&lt;T&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()