import './index.css'
import { useState, useEffect } from 'react'
import { Session, createClient } from '@supabase/supabase-js'
import { Auth } from '@supabase/auth-ui-react'
import { ThemeSupa } from '@supabase/auth-ui-shared'

const anonKey = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImhpdmZyZWZpd3Vsdmxoc2poem9wIiwicm9sZSI6ImFub24iLCJpYXQiOjE3MDYyMjcyNzEsImV4cCI6MjAyMTgwMzI3MX0.6edQB6SxlAcb-C-4t20jHCrKUq75h1Fz0cD1i6GW1tI";
const supabase = createClient("https://hivfrefiwulvlhsjhzop.supabase.co", anonKey);

export default function Login() {
    const [session, setSession] = useState<Session | null>(null)

    useEffect(() => {
        supabase.auth.getSession().then(({ data: { session } }) => {
            setSession(session)
        })

        const {
            data: { subscription },
        } = supabase.auth.onAuthStateChange((_event, session) => {
            setSession(session)
        })

        return () => subscription.unsubscribe()
    }, [])

    const handleCopyToken = () => {
        if (session) {
            const jwtToken = session.access_token;

            // Create a textarea element
            const textarea = document.createElement('textarea');
            textarea.value = jwtToken;

            // Append the textarea to the body
            document.body.appendChild(textarea);

            // Select the text inside the textarea
            textarea.select();
            document.execCommand('copy');

            // Remove the textarea from the body
            document.body.removeChild(textarea);

            // Notify the user that the token is copied
            alert('JWT Token copied to clipboard!');
        }
    };



    if (!session) {
        return (<Auth supabaseClient={supabase} appearance={{ theme: ThemeSupa }} providers={[]} />)
    }
    else {
        const jwtToken = session.access_token;
        console.log('JWT Token:', jwtToken);
        return (<div>
            <div>Logged in!</div>
            <div style={{ position: 'relative' }}>
                <textarea
                    readOnly
                    value={session.access_token}
                    style={{ width: '100%', height: '100px' }}
                />
                <button
                    onClick={handleCopyToken}
                    style={{ position: 'absolute', top: '5px', right: '5px' }}
                >
                    Copy
                </button>
            </div>
        </div>
        )
    }
}